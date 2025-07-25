<script lang="ts">
  import { CalendarDays, FileText, X } from "@lucide/svelte";
  import type { PageProps } from "./$types";
  import { navbarState } from "$lib/stores/state.svelte";
  import { formatDueDate } from "$lib/utils";
  import { applyAction, deserialize } from "$app/forms";
  import type { ActionResult } from "@sveltejs/kit";
  import { invalidateAll } from "$app/navigation";
  import { page } from "$app/state";
  import { download_material_file } from "$lib/api/classrooms";

  let { data }: PageProps = $props();

  navbarState.assignmentName = data.title;
  navbarState.currentPage = "assignment";
  navbarState.classPath = "/class/" + data.class_id;

  let tab = $state<"instructions" | "yourwork">("instructions");

  let files = $state<FileList>();
  let selectedFilesArray = $state<File[]>([]);

  let filesToShow = $state(data.submission_files);

  let loading = $state(false);
  let canTurnIn = $derived(filesToShow.length > 0);

  // svelte-ignore non_reactive_update
  let form: HTMLFormElement;

  function handleFileChange(event: Event) {
    const newFiles = Array.from(files || []).filter(
      (file) => file instanceof File
    ) as File[];
    for (const file of newFiles) {
      if (
        selectedFilesArray.some((f) => f.name === file.name) ||
        filesToShow.some((f) => f.file_name === file.name)
      ) {
        console.warn(`File ${file.name} is already selected.`);
        continue;
      }
      selectedFilesArray.push(file);
      filesToShow.push({
        id: null,
        file_name: file.name,
        content_type: file.type,
        file_type: "submission",
      });
    }
  }

  function removeFile(index: number, file_id: string | null) {
    loading = true;
    selectedFilesArray.splice(index, 1);
    filesToShow.splice(index, 1);
    loading = false;
  }

  async function handleSubmit(
    event: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }
  ) {
    event.preventDefault();
    loading = true;

    if (event.submitter?.getAttribute("formaction")) {
      // DELETING FILE OR CANCELING SUBMISSION
      const actionUrl =
        page.url.origin +
        page.url.pathname +
        event.submitter.getAttribute("formaction");

      const formData = new FormData();
      const fileId = event.submitter.getAttribute("id") || "";
      const fileName = event.submitter.getAttribute("name");

      if (fileId == "" && fileName != null) {
        console.log("IM HERE SUCCESS DELETING FROM null");
        const showIndex = filesToShow.findIndex(
          (file) => file.file_name === fileName
        );
        if (showIndex !== -1) {
          filesToShow.splice(showIndex, 1);
        }

        const selectedIndex = selectedFilesArray.findIndex(
          (file) => file.name === fileName
        );
        if (selectedIndex !== -1) {
          selectedFilesArray.splice(selectedIndex, 1);
        }
        loading = false;
      } else {
        formData.append("fileId", fileId);

        const response = await fetch(actionUrl, {
          method: "POST",
          body: formData,
        });

        const result: ActionResult = deserialize(await response.text());

        if (result.type === "success") {
          await invalidateAll();
        }

        applyAction(result);
        loading = false;
      }
    } else {
      // file upload
      const formData = new FormData();
      for (const file of selectedFilesArray) {
        formData.append("fileupload", file);
      }
      const response = await fetch(event.currentTarget.action, {
        method: "POST",
        body: formData,
      });
      const result: ActionResult = deserialize(await response.text());

      loading = false;

      if (result.type === "success") {
        await invalidateAll();
      }

      applyAction(result);
    }
  }

  function formatFileType(mimeType: string): string {
    switch (mimeType) {
      case "application/pdf":
        return "PDF";
      case "application/msword":
        return "DOC";
      case "application/vnd.openxmlformats-officedocument.wordprocessingml.document":
        return "DOCX";
      case "text/plain":
        return "TXT";
      case "image/jpeg":
        return "JPG";
      case "image/png":
        return "PNG";
      default:
        return mimeType.split("/").pop()?.toUpperCase() || mimeType;
    }
  }

  async function downloadFile(fileId: string | null, fileName: string) {
    if (fileId == null || fileId === "") return;
    try {
      const response = await fetch(
        download_material_file(data.class_id, fileId)
      );
      if (response.ok) {
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.style.display = "none";
        a.href = url;
        a.download = fileName;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        document.body.removeChild(a);
      }
    } catch (error) {
      console.error("Failed to download file:", error);
    }
  }

  $effect(() => {
    filesToShow = data.submission_files;
    selectedFilesArray = [];
  });
</script>

<div class="w-full h-full flex justify-center p-6">
  <div
    class="bg-black/50 box-shadow-lg backdrop-blur-sm p-3 rounded-2xl w-full max-w-screen-xl"
  >
    <!-- Main White Content -->
    <div class="bg-white/70 rounded-2xl p-6">
      <div class="flex justify-between items-start mb-4">
        <div class="text-gray-900 space-y-1">
          <div class="flex items-center gap-2">
            <div class="text-black drop-shadow">
              <h1 class="text-3xl font-bold">📌{data.title}</h1>
              <p class="text-md text-gray-700">{data.class_title}</p>
            </div>
          </div>
        </div>
        <div class="text-right text-sm space-y-1">
          <div class="text-lg font-semibold">{data.points} points</div>
          <div class="flex justify-end">
            {#if data.is_submitted}
              {#if data.grade !== null}
                <div
                  class="bg-green-500/50 text-white font-medium px-2 py-1 rounded w-max"
                >
                  Graded: {data.grade}/{data.points} points
                </div>
              {:else}
                <div
                  class="bg-yellow-500/50 text-black font-medium px-2 py-1 rounded w-max"
                >
                  Submitted, awaiting grade
                </div>
              {/if}
            {:else}
              <div
                class="bg-gray-500/50 text-black font-medium px-2 py-1 rounded w-max"
              >
                Not submitted
              </div>
            {/if}
          </div>
          <div
            class="flex text-sm text-gray-700 items-center gap-1 justify-end"
          >
            <CalendarDays size="18" />
            {formatDueDate(data.due_date)}
          </div>
        </div>
      </div>

      <!-- Tab Switcher -->
      <div class="flex gap-4 mb-4">
        <button
          onclick={() => (tab = "instructions")}
          class={`px-4 py-2 rounded-xl flex items-center gap-1 shadow ${
            tab === "instructions"
              ? "bg-lime-500 text-white shadow"
              : "bg-white/50 hover:shadow-white/50 transition-shadow duration-300"
          }`}
        >
          📘 Instructions
        </button>

        <button
          onclick={() => (tab = "yourwork")}
          class={`px-4 py-2 rounded-xl flex items-center gap-1 shadow ${
            tab === "yourwork"
              ? "bg-lime-500 text-white shadow"
              : "bg-white/50 hover:shadow-white/50 transition-shadow duration-300"
          }`}
        >
          📂 Your Work
        </button>
      </div>

      <!-- Tab Content -->
      {#if tab === "instructions"}
        <div class="bg-white/80 p-6 rounded-xl shadow mb-6">
          <p class="text-lg whitespace-pre-line">
            {data.description}
          </p>
        </div>

        <div class="bg-white/80 p-6 rounded-xl shadow">
          <h2 class="text-lg font-semibold mb-4">Materials</h2>
          <div class="space-y-3">
            {#each data.materials as material}
              {@render material_card(
                material.id ?? "",
                material.file_name,
                material.content_type
              )}
            {/each}
          </div>
        </div>
      {/if}

      {#if tab === "yourwork"}
        <form
          bind:this={form}
          method="POST"
          action={`?/uploadassignmentfiles`}
          onsubmit={handleSubmit}
          class="space-y-4 mt-2"
        >
          <div class="bg-white/80 p-6 rounded-xl shadow text-center">
            <h2 class="text-lg font-semibold mb-4 text-left">
              Your submission
            </h2>
            <div
              class="flex flex-col items-center justify-center h-40 border-2 border-dashed border-gray-300 rounded-xl bg-white/60 overflow-y-auto p-2"
            >
              {#if filesToShow.length > 0}
                <div class="text-gray-700 w-full">
                  <div class="flex flex-wrap gap-2 w-full">
                    {#each filesToShow as file, index}
                      {@render file_card(
                        index,
                        file.id,
                        file.file_name,
                        file.content_type
                      )}
                    {/each}
                  </div>
                </div>
              {:else}
                <div class="text-gray-500">No files selected</div>
              {/if}
            </div>
            <div class="flex justify-between mt-4">
              <label
                class="px-4 py-2 bg-white border rounded shadow hover:shadow-md {loading ||
                data.is_submitted
                  ? 'opacity-50 cursor-not-allowed'
                  : 'hover:bg-gray-100'}"
                for="fileupload">Add files</label
              >
              <input
                bind:files
                hidden
                id="fileupload"
                name="fileupload"
                multiple
                type="file"
                accept=".pdf,.doc,.docx,.txt,.jpg,.jpeg,.png"
                onchange={handleFileChange}
                disabled={loading || data.is_submitted}
              />
              {#if !data.is_submitted}
                <button
                  type="submit"
                  class={`px-4 py-2 rounded font-semibold shadow transition ${
                    canTurnIn && !loading
                      ? "bg-lime-500 text-white hover:bg-lime-600"
                      : "bg-gray-300 text-white opacity-50 cursor-not-allowed"
                  }`}
                  disabled={!canTurnIn || loading}
                >
                  Turn in
                </button>
              {:else}
                <button
                  formaction="?/cancelsubmission"
                  type="submit"
                  class={`px-4 py-2 rounded font-semibold shadow transition ${
                    canTurnIn && !loading && data.grade === null
                      ? "bg-red-500 text-white hover:bg-red-600"
                      : "bg-gray-300 text-white opacity-50 cursor-not-allowed"
                  }`}
                  disabled={loading || data.grade !== null}
                >
                  Cancel submission
                </button>
              {/if}
            </div>
          </div>
        </form>
      {/if}
    </div>
  </div>
</div>

{#snippet file_card(
  index: number,
  file_id: string | null,
  file_name: string,
  file_type: string
)}
  <div
    class="bg-white p-3 rounded-xl flex items-center justify-between shadow w-full max-w-sm"
  >
    <div class="flex items-center gap-3">
      <FileText size="20" />
      <div class="truncate">
        <div class="font-medium break-words max-w-3xs truncate">
          {file_name}
        </div>
        <div class="text-sm text-gray-600">
          {formatFileType(file_type)}
        </div>
      </div>
    </div>
    {#if !loading && !data.is_submitted && data.grade !== null}
      <button
        type="submit"
        id={file_id ? file_id : ""}
        name={file_name}
        formaction="?/deleteassignmentfile"
        class="text-red-500 hover:text-red-700 transition-colors"
      >
        <X size="20" />
      </button>
    {:else}
      <div class="text-gray-400">
        <X size="20" />
      </div>
    {/if}
  </div>
{/snippet}

{#snippet material_card(file_id: string, file_name: string, file_type: string)}
  <div class="bg-white p-4 rounded-xl flex items-center justify-between shadow hover:scale-[1.01] hover:bg-white/50 transition block cursor-pointer"
  onclick={() => downloadFile(file_id, file_name)}
  role="button"
  tabindex=69
  >
    <div class="flex items-center gap-3">
      <FileText size="20" />
      <div>
        <div class="font-medium">{file_name}</div>
        <div class="text-sm text-gray-600">{formatFileType(file_type)}</div>
      </div>
    </div>
  </div>
{/snippet}
