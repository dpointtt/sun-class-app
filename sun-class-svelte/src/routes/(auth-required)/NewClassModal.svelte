<script lang="ts">
  import Modal from "./Modal.svelte";
  import { showJoinCreateModal } from "$lib/stores/modalState";
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  let tab: "join" | "create" = $state("join");
  let name = $state("");
  let description = $state("");
  let joinCode = $state("");

  let loading = $state(false);
  let canCreate = $derived(name.trim() !== "" && description.trim() !== "");
  let canJoin = $derived(joinCode.trim() !== "");

  const closeModal = () => showJoinCreateModal.set(false);

  const switchTab = (newTab: "join" | "create") => {
    tab = newTab;
    name = "";
    description = "";
    joinCode = "";
  };
</script>

{#if $showJoinCreateModal}
  <Modal onClose={closeModal}>
    <div class="space-y-4 mt-2">
      <div class="flex justify-center gap-4">
        <button
          class={`px-4 py-2 rounded ${tab === "join" ? "bg-lime-500 text-white" : "bg-gray-300"}`}
          onclick={() => {switchTab("join")}}>Join Class</button
        >
        <button
          class={`px-4 py-2 rounded ${tab === "create" ? "bg-lime-500 text-white" : "bg-gray-300"}`}
          onclick={() => {switchTab("create")}}>Create Class</button
        >
      </div>

      {#if tab === "join"}
        <form
          method="POST"
          action="?/joinclass"
          use:enhance={() => {
            loading = true;

            return async ({ update, result }) => {
              if (result.type === "success") {
                await update();
                showJoinCreateModal.set(false);
                loading = false;
              }
            };
          }}
        >
          <label class="block mb-1 text-sm">Join Code</label>
          <input
            type="text"
            name="joinCode"
            bind:value={joinCode}
            disabled={loading}
            class={`w-full mb-3 px-4 py-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-lime-400 transition 
                  ${
                    !loading
                      ? "bg-white/80 text-gray-900"
                      : "bg-gray-100 text-gray-600 cursor-not-allowed"
                  }`}
            required
          />
          <button
            class={`px-4 py-2 rounded font-semibold shadow transition 
                  ${
                    canJoin && !loading
                      ? "bg-lime-500 text-white hover:bg-lime-600"
                      : "bg-gray-300 text-white opacity-50 cursor-not-allowed hover:cursor-default"
                  }`}
            disabled={!canJoin || loading}>
            Join
          </button>
        </form>
      {:else}
        <form
          method="POST"
          action="?/createclass"
          use:enhance={() => {
            loading = true;
            return async ({ update, result }) => {
              if (result.type === "success") {
                await update();
                showJoinCreateModal.set(false);
                loading = false;
              }
            };
          }}
        >
          <label class="block mb-1 text-sm">Class Name</label>
          <input
            type="text"
            name="title"
            bind:value={name}
            disabled={loading}
            class={`w-full mb-3 px-4 py-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-lime-400 transition 
                  ${
                    !loading
                      ? "bg-white/80 text-gray-900"
                      : "bg-gray-100 text-gray-600 cursor-not-allowed"
                  }`}
            required
          />

          <label class="block mb-1 text-sm">Description</label>
          <textarea
            name="description"
            bind:value={description}
            disabled={loading}
            class={`w-full mb-3 px-4 py-2 rounded-lg border focus:outline-none focus:ring-2 focus:ring-lime-400 transition 
                  ${
                    !loading
                      ? "bg-white/80 text-gray-900"
                      : "bg-gray-100 text-gray-600 cursor-not-allowed"
                  }`}
            required
          ></textarea>

          <button
            class={`px-4 py-2 rounded font-semibold shadow transition 
                  ${
                    canCreate && !loading
                      ? "bg-lime-500 text-white hover:bg-lime-600"
                      : "bg-gray-300 text-white opacity-50 cursor-not-allowed hover:cursor-default"
                  }`}
            disabled={!canCreate || loading}>
            Create
          </button>
        </form>
      {/if}
    </div>
  </Modal>
{/if}
