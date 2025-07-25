<script lang="ts">
  import { navbarState } from "$lib/stores/state.svelte";
  import { Download, X, Check } from "@lucide/svelte";
  import type { PageProps } from "./$types";
  import { download_submission_file } from "$lib/api/classrooms";
  import { enhance } from "$app/forms";

  let { data }: PageProps = $props();

  navbarState.currentPage = "submission";
  navbarState.classPath = "/class/" + data.class_id + "/submissions";

  let originalGrade = $state(data.submission_data.grade);
  let grade = $state(originalGrade);

  let saving = $state(false);

  async function downloadFile(fileId: string | null, fileName: string) {
    if (fileId == null) return;
    try {
      const response = await fetch(
        download_submission_file(data.class_id, fileId)
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
    originalGrade = data.submission_data.grade;
    grade = originalGrade;
  });
</script>

<div class="w-full h-full flex justify-center p-6">
  <div
    class="bg-black/50 box-shadow-lg backdrop-blur-sm p-3 rounded-2xl w-full max-w-screen-md"
  >
    <div class="bg-white/70 rounded-2xl p-6 space-y-8">
      <div>
        <h1 class="text-3xl font-bold mb-1">Submission</h1>
        <p class="text-gray-600 text-lg">
          Assignment: {data.submission_data.assignment_title}
        </p>
        <p class="text-gray-600 text-lg">
          Submitted by {data.submission_data.student_name}
        </p>
      </div>

      <div>
        <h2 class="text-xl font-semibold mb-4">Submitted Files</h2>
        {#if data.submission_data.files.length > 0}
          <div class="space-y-3">
            {#each data.submission_data.files as file}
              <div
                class="bg-white/80 p-3 rounded-lg shadow flex justify-between items-center"
              >
                <p class="text-gray-800 font-medium">{file.file_name}</p>
                <button
                  onclick={() => downloadFile(file.id, file.file_name)}
                  class="text-sm text-blue-600 hover:underline"
                >
                  <Download size="20" />
                </button>
              </div>
            {/each}
          </div>
        {:else}
          <p class="text-gray-500">No files submitted.</p>
        {/if}
      </div>

      <div>
        <h2 class="text-xl font-semibold mb-4">
          Grade ({data.submission_data.assignment_points} max.)
        </h2>

        <form
          method="POST"
          action="?/savegrade"
          class="flex flex-col sm:flex-row items-start sm:items-center gap-4"
          use:enhance={() => {
            saving = true;
            return async ({ update }) => {
              update({ reset: false });
              saving = false;
            };
          }}
        >
          <input
            type="number"
            min="0"
            max={data.submission_data.assignment_points}
            step="1"
            bind:value={grade}
            name="grade"
            class="w-28 rounded-xl border border-gray-300 px-4 py-2 bg-white/90 text-lg font-semibold focus:outline-none focus:ring-2 focus:ring-lime-400"
            disabled={saving || originalGrade !== null}
          />

          {#if originalGrade === null || !data.submission_data.is_graded}
            <button
              type="submit"
              disabled={saving}
              class="bg-lime-500 hover:bg-lime-600 text-white px-5 py-2.5 rounded-xl shadow disabled:opacity-50 transition flex items-center gap-2"
            >
              <Check size="18" />
              {saving ? "Saving..." : "Save"}
            </button>
          {:else}
            <button
              formaction="?/cancelgrade"
              class="bg-red-500 hover:bg-red-600 text-white px-5 py-2.5 rounded-xl shadow transition flex items-center gap-2"
              disabled={saving}
            >
              <X size="18" /> Cancel Grade
            </button>
          {/if}
        </form>
      </div>
    </div>
  </div>
</div>
