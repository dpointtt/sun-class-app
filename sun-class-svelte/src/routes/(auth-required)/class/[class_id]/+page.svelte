<script lang="ts">
  import {
    BookOpenText,
    Users,
    ClipboardList,
    Plus,
    Book,
    Upload,
    X,
  } from "@lucide/svelte";

  import type { PageProps } from "./$types";
  import { navbarState } from "$lib/stores/state.svelte";

  import type { Assignment } from "$lib/types";
  import Modal from "../../Modal.svelte";
  import { tick } from "svelte";
  import { formatDueDate } from "$lib/utils";
  import { page } from "$app/state";
  import { goto, invalidateAll } from "$app/navigation";
  import {
    create_assignment,
    save_assignment_materials,
  } from "$lib/api/classrooms";

  let { data }: PageProps = $props();

  navbarState.className = data.class.title;
  navbarState.currentPage = "class";

  let tab = $state<"stream" | "classwork" | "people">("stream");

  const backgroundImage = "/default1.jpg";

  let showCreateAssignmentModal = $state(false);

  let title = $state("");
  let description = $state("");
  let due_date = $state("");
  let max_points = $state("");
  let selectedFiles = $state<FileList | null>(null);

  let loading = $state(false);
  let canCreateAssignment = $derived(
    title.trim() !== "" &&
      description.trim() !== "" &&
      due_date !== "" &&
      max_points !== ""
  );

  const closeCreateAssignmentModal = () => {
    showCreateAssignmentModal = false;
    title = "";
    description = "";
    due_date = "";
    max_points = "";
    selectedFiles = null;
  };

  let copied = $state(false);

  async function copyCode(code: string) {
    try {
      await navigator.clipboard.writeText(code);
      copied = true;
      await tick();
      setTimeout(() => (copied = false), 1500);
    } catch (err) {
      console.error("Failed to copy!", err);
    }
  }

  const upcomingAssignments = $derived(
    data.class.assignments.filter((a) => new Date(a.due_date) > new Date())
  );

  async function handleCreateAssignment(
    event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }
  ) {
    event.preventDefault();

    if (!canCreateAssignment || loading) return;

    loading = true;
    const assignmentResponse = await fetch(
      create_assignment(data.class.id, title, description, due_date, +max_points)
    )
      .then((response) => response.json())
      .catch(console.error);

    if (selectedFiles && selectedFiles.length > 0) {
      const formData = new FormData();
      Array.from(selectedFiles).forEach((file, index) => {
        formData.append(`file_${index}`, file);
      });
      const materialsResponse = await fetch(
        save_assignment_materials(data.class.id, assignmentResponse.id, formData)
      );
      if (materialsResponse.ok) {
        await invalidateAll();
      } else {
        console.log(materialsResponse);
      }
      loading = false;
      closeCreateAssignmentModal();
    }
  }

  // Remove selected file
  function removeFile(index: number) {
    if (selectedFiles) {
      const dt = new DataTransfer();
      const files = Array.from(selectedFiles);
      files.splice(index, 1);
      files.forEach((file) => dt.items.add(file));
      selectedFiles = dt.files;
    }
  }
</script>

<div class="w-full h-full flex justify-center p-6">
  <div
    class="bg-black/50 box-shadow-lg backdrop-blur-sm p-3 rounded-2xl w-full max-w-screen-xl"
  >
    <!-- Background Header Image -->
    <div
      class="h-48 w-full bg-cover bg-center rounded-t-2xl relative"
      style="background-image: url({backgroundImage});"
    >
      <div class="absolute inset-0 bg-black/30 rounded-2xl"></div>
      <div class="absolute bottom-4 left-6 text-white drop-shadow">
        <h1 class="text-3xl font-bold">➗{data.class.title}</h1>
        <p class="text-md text-white/90">{data.class.teacher}</p>
      </div>
    </div>

    <!-- Main White Content -->
    <div class="bg-white/70 rounded-b-2xl p-6">
      <div class="flex justify-between items-center mb-4">
        <div class="flex gap-4">
          <button
            class="px-4 py-2 rounded-xl flex items-center gap-1 shadow {tab ===
            'stream'
              ? 'bg-lime-500 text-white shadow'
              : 'bg-white/50 hover:shadow-white/50 transition-shadow duration-300'}"
            onclick={() => (tab = "stream")}
          >
            <BookOpenText size="16" /> Stream
          </button>
          <button
            class="px-4 py-2 rounded-xl flex items-center gap-1 shadow {tab ===
            'classwork'
              ? 'bg-lime-500 text-white shadow'
              : 'bg-white/50 hover:shadow-white/50 transition-shadow duration-300'}"
            onclick={() => (tab = "classwork")}
          >
            <ClipboardList size="16" /> Classwork
          </button>
          <button
            class="px-4 py-2 rounded-xl flex items-center gap-1 shadow {tab ===
            'people'
              ? 'bg-lime-500 text-white shadow'
              : 'bg-white/50 hover:shadow-white/50 transition-shadow duration-300'}"
            onclick={() => (tab = "people")}
          >
            <Users size="16" /> People
          </button>
        </div>
        {#if data.is_teacher}
          <button
            onclick={() => goto(page.url.pathname + "/submissions")}
            class="px-4 py-2 rounded-xl flex items-center gap-1 shadow bg-white/50 hover:shadow-white/50 transition-shadow duration-300 ms-3 hover:no-underline"
          >
            <Book size="16" class="hover:no-underline" /> View Submissions
          </button>
          <button
            class="bg-lime-500 hover:bg-lime-600 text-white px-4 py-2 rounded-xl shadow flex items-center gap-1 transition duration-200"
            onclick={() => (showCreateAssignmentModal = true)}
          >
            <Plus size="16" /> Create Assignment
          </button>
        {/if}
      </div>

      {#if tab === "stream"}
        <div class="flex flex-col lg:flex-row gap-6">
          <div class="flex-1 space-y-4">
            <div class="bg-gray-100 p-4 rounded-xl shadow">
              Welcome to the class! 🎉
            </div>
            {#each data.class.assignments as assignment}
              <a
                href={`/class/${data.class.id}/assignment/${assignment.id}`}
                class="hover:scale-[1.01] transition-transform block"
              >
                {@render assignment_block(assignment)}
              </a>
            {/each}
          </div>

          <div class="w-full lg:w-1/3 space-y-4">
            {#if data.class.assignments.length != 0}
              <div class="bg-white/80 p-4 rounded-xl shadow">
                <p class="font-semibold mb-2">Upcoming</p>
                {#each upcomingAssignments as assignment}
                  <a
                    href={`/class/${data.class.id}/assignment/${assignment.id}`}
                    class="hover:scale-[1.01] transition-transform block"
                  >
                    <div class="text-sm text-gray-700">
                      📌 {assignment.title} –
                      <span class="text-gray-500"
                        >{formatDueDate(assignment.due_date)}</span
                      >
                    </div>
                  </a>
                {/each}
              </div>
            {/if}
            <div class="bg-white/80 p-4 rounded-xl shadow text-center">
              <p class="font-semibold">Share this code with your students</p>
              <div
                class="bg-gray-200 text-center mt-2 p-2 rounded font-mono cursor-pointer hover:bg-gray-300 transition duration-300"
                onclick={() => copyCode(data.class.join_code)}
                title="Click to copy"
              >
                {#if copied}
                  ✅ Copied!
                {:else}
                  {data.class.join_code}
                {/if}
              </div>
            </div>
          </div>
        </div>
      {/if}

      {#if tab === "classwork"}
        <div class="space-y-4">
          {#each data.class.assignments as assignment}
            <a
              href={`/class/${data.class.id}/assignment/${assignment.id}`}
              class="hover:scale-[1.01] transition-transform block"
            >
              {@render assignment_block(assignment)}
            </a>
          {/each}
        </div>
      {/if}

      {#if tab === "people"}
        <div class="space-y-6">
          <div>
            <p class="font-semibold text-lg mb-2">Teachers</p>
            {#each data.class.users as s}
              {#if s.role === "teacher"}
                <div class="bg-gray-100 p-3 rounded-xl shadow">
                  👨‍🏫 {s.name}
                </div>
              {/if}
            {/each}
          </div>
          <div>
            <p class="font-semibold text-lg mb-2">Students</p>
            <div class="space-y-2">
              {#each data.class.users as s}
                {#if s.role === "student"}
                  <div class="bg-gray-100 p-3 rounded-xl shadow">
                    👤 {s.name}
                  </div>
                {/if}
              {/each}
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

{#if showCreateAssignmentModal}
  <Modal onClose={() => closeCreateAssignmentModal()}>
    <form onsubmit={handleCreateAssignment} class="space-y-4 mt-2">
      <h2 class="text-xl font-semibold">Create Assignment</h2>

      <div>
        <label class="block text-sm font-medium mb-1">Title</label>
        <input
          type="text"
          bind:value={title}
          disabled={loading}
          required
          class={`w-full px-4 py-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-lime-400 transition ${
            !loading ? "bg-white/80" : "bg-gray-100 text-gray-600"
          }`}
        />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">Description</label>
        <textarea
          bind:value={description}
          disabled={loading}
          required
          rows="3"
          class={`w-full px-4 py-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-lime-400 transition ${
            !loading ? "bg-white/80" : "bg-gray-100 text-gray-600"
          }`}
        ></textarea>
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">Due Date</label>
        <input
          type="datetime-local"
          bind:value={due_date}
          disabled={loading}
          required
          class={`w-full px-4 py-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-lime-400 transition ${
            !loading ? "bg-white/80" : "bg-gray-100 text-gray-600"
          }`}
        />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">Max Points</label>
        <input
          type="number"
          bind:value={max_points}
          min="1"
          disabled={loading}
          required
          class={`w-full px-4 py-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-lime-400 transition ${
            !loading ? "bg-white/80" : "bg-gray-100 text-gray-600"
          }`}
        />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1"
          >Assignment Materials (Optional)</label
        >
        <div class="space-y-2">
          <label
            class={`flex items-center justify-center w-full h-32 border-2 border-dashed border-gray-300 rounded-lg cursor-pointer hover:bg-gray-50 transition ${
              loading ? "opacity-50 cursor-not-allowed" : ""
            }`}
          >
            <div class="flex flex-col items-center">
              <Upload size="24" class="text-gray-400 mb-2" />
              <span class="text-sm text-gray-500">Click to upload files</span>
            </div>
            <input
              type="file"
              multiple
              bind:files={selectedFiles}
              disabled={loading}
              class="hidden"
              accept=".pdf,.doc,.docx,.txt,.png,.jpg,.jpeg,.gif"
            />
          </label>

          {#if selectedFiles && selectedFiles.length > 0}
            <div class="space-y-2">
              <p class="text-sm font-medium">Selected Files:</p>
              {#each Array.from(selectedFiles) as file, index}
                <div
                  class="flex items-center justify-between bg-gray-100 p-2 rounded"
                >
                  <span class="text-sm text-gray-700">{file.name}</span>
                  <button
                    type="button"
                    onclick={() => removeFile(index)}
                    disabled={loading}
                    class="text-red-500 hover:text-red-700 transition"
                  >
                    <X size="16" />
                  </button>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-4">
        <button
          type="button"
          onclick={closeCreateAssignmentModal}
          disabled={loading}
          class="px-4 py-2 rounded font-semibold border border-gray-300 hover:bg-gray-50 transition"
        >
          Cancel
        </button>
        <button
          type="submit"
          class={`px-4 py-2 rounded font-semibold shadow transition ${
            canCreateAssignment && !loading
              ? "bg-lime-500 text-white hover:bg-lime-600"
              : "bg-gray-300 text-white opacity-50 cursor-not-allowed"
          }`}
          disabled={!canCreateAssignment || loading}
        >
          {loading ? "Creating..." : "Create Assignment"}
        </button>
      </div>
    </form>
  </Modal>
{/if}

{#snippet assignment_block(assignment: Assignment)}
  <div class="bg-white/80 p-4 rounded-xl shadow flex flex-col gap-1">
    <div class="text-lg font-semibold flex items-center gap-2">
      📌 {assignment.title}
    </div>
    <div class="text-sm text-gray-600">Due: {assignment.due_date}</div>
  </div>
{/snippet}
