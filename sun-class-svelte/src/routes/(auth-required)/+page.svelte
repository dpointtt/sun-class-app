<script lang="ts">
  import { navbarState, classTab } from "$lib/stores/state.svelte";
  import type { PageProps } from "./$types";

  navbarState.currentPage = "classes";

  let { data }: PageProps = $props();
</script>

<div>
  <div
    class="flex gap-4 mb-4 bg-black/50 box-shadow-lg backdrop-blur-sm p-3 rounded-2xl w-max"
  >
    <button
      onclick={() => classTab.set("enrolled")}
      class="px-4 py-2 rounded-xl shadow {$classTab !== 'enrolled'
        ? 'bg-white/50'
        : 'bg-white/80'}"
    >
      Enrolled
    </button>
    <button
      onclick={() => classTab.set("teaching")}
      class="px-4 py-2 rounded-xl shadow {$classTab !== 'teaching'
        ? 'bg-white/50'
        : 'bg-white/80'}"
    >
      Teaching
    </button>
  </div>

  <div
    class="grid grid-cols-[repeat(auto-fill,_minmax(300px,_1fr))] gap-[30px] max-w-full"
  >
    {#if $classTab === "enrolled"}

      {#each data.enrolled_classes as c}
        {@render class_card(
          c.id,
          c.title,
          c.teacher,
          "🏫",
          "/default1.jpg",
          []
        )}
      {/each}
      
    {:else}
    
      {#each data.teaching_classes as c}
        {@render class_card(
          c.id,
          c.title,
          c.teacher,
          "🏫",
          "/default1.jpg",
          []
        )}
      {/each}

    {/if}
  </div>
</div>

{#snippet class_card(
  id: string,
  title: string,
  teacher: string,
  emoji: string,
  backgroundImage: string,
  assignments: string[]
)}
  <div
    class="bg-black/50 box-shadow-lg backdrop-blur-sm p-3 rounded-2xl h-max w-max flex"
  >
    <div
      class="bg-white/70 rounded-2xl shadow overflow-hidden size-[300px] max-w-xs hover:shadow-lg transition-shadow duration-300"
    >
      <!-- Header with background image -->
      <div
        class="group relative h-32 bg-cover bg-center hover:cursor-pointer"
        style="background-image: url({backgroundImage});"
      >
        <div class="absolute inset-0 bg-black/30"></div>
        <div class="absolute bottom-2 left-3 text-white drop-shadow">
          <a
            href="/class/{id}"
            class="text-white text-lg font-semibold flex items-center gap-1 group-hover:underline"
          >
            {emoji}{title}
          </a>
          <div class="text-sm">{teacher}</div>
        </div>
      </div>

      <!-- Assignments -->
      <div class="p-4 text-sm text-gray-800 space-y-2">
        <div class="font-semibold">Upcoming:</div>
        {#if assignments.length > 0}
          {#each assignments as a, i}
            <a
              href="/class/{id}/assignment/1"
              class="text-gray-700 hover:underline block">📌 {a}</a
            >
          {/each}
        {:else}
          <div class="text-gray-500">No upcoming assignments 🎉</div>
        {/if}
      </div>
    </div>
  </div>
{/snippet}
