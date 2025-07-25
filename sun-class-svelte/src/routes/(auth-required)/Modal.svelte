<script lang="ts">
  import { X } from "@lucide/svelte";

  export let onClose: () => void;
</script>

<div
  class="fixed inset-0 bg-black/50 flex justify-center items-center z-50"
  on:click={() => onClose()}
>
  <div class="bg-white rounded-xl shadow-lg p-6 w-full max-w-md relative" on:click|stopPropagation>
    <slot />
    <button class="absolute top-2 right-2 text-gray-500 hover:text-gray-700" on:click={() => onClose()}><X size="24" /></button>
  </div>
</div>