<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { ChevronLeft, ChevronDown, Plus, User } from "@lucide/svelte";
  import { navbarState } from "$lib/stores/state.svelte";
  import { logout } from "$lib/api/auth";
  import { showJoinCreateModal } from "$lib/stores/modalState";

  let profileOpen = false;
  let menuOpen = false;
  let profileRef: HTMLDivElement;
  let menuRef: HTMLDivElement;

  const handleClickOutside = (event: MouseEvent) => {
    if (profileRef && !profileRef.contains(event.target as Node)) {
      profileOpen = false;
    }
    if (menuRef && !menuRef.contains(event.target as Node)) {
      menuOpen = false;
    }
  };

  onMount(() => {
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  });

  const handleLogout = async () => {
    await logout();
    goto("/login");
  };
</script>

<nav
  class="w-full flex justify-between items-center px-6 py-2 fixed top-0 left-0 z-50 bg-black/50 box-shadow-lg backdrop-blur-sm p-2 rounded-b-2xl"
>
  <!-- Left: Logo -->
  <a
    href="/"
    class="bg-white/60 rounded-2xl px-4 py-2 shadow flex items-center gap-2 hover:underline hover:shadow-white/50"
  >
    <img src="/logo_sunflower.png" alt="Logo" class="block w-10 h-auto" />
    <span class="font-semibold text-lg">SunClass</span>
  </a>

  <!-- Middle: dynamic -->
  <div class="flex items-center gap-2 text-lg font-medium">
    {#if navbarState.currentPage === "class"}
      <div
        class="flex items-center gap-2 bg-white/60 rounded-2xl px-4 py-2 shadow hover:shadow-white/50"
      >
        <ChevronLeft
          size={20}
          class="cursor-pointer"
          onclick={() => goto("/")}
        />
        <span>{navbarState.className}</span>
      </div>
    {:else if navbarState.currentPage === "assignment"}
      <div
        class="flex items-center gap-2 bg-white/60 rounded-2xl px-4 py-2 shadow hover:shadow-white/50"
      >
        <ChevronLeft
          size={20}
          class="cursor-pointer"
          onclick={() => goto(navbarState.classPath)}
        />
        <span>{navbarState.assignmentName}</span>
      </div>
    {:else if navbarState.currentPage === "submissions"}
      <div
        class="flex items-center gap-2 bg-white/60 rounded-2xl px-4 py-2 shadow hover:shadow-white/50"
      >
        <ChevronLeft
          size={20}
          class="cursor-pointer"
          onclick={() => goto(navbarState.classPath)}
        />
        <span>{navbarState.className}</span>
      </div>
    {:else if navbarState.currentPage === "submission"}
      <div
        class="flex items-center gap-2 bg-white/60 rounded-2xl px-4 py-2 shadow hover:shadow-white/50"
      >
        <ChevronLeft
          size={20}
          class="cursor-pointer"
          onclick={() => goto(navbarState.classPath)}
        />
        <span>Submissions</span>
      </div>
    {:else}
      <div class="relative" bind:this={menuRef}>
        <button
          class="bg-white/60 rounded-2xl px-6 py-2 shadow cursor-pointer select-none text-lg hover:shadow-white/50 whitespace-nowrap"
          on:click={() => (menuOpen = !menuOpen)}
        >
          <span class="font-medium flex items-center gap-1">
            {#if navbarState.currentPage === "classes"}Classes📖{/if}
            {#if navbarState.currentPage === "profile"}Classes📖{/if}
            {#if navbarState.currentPage === "grades"}Grades💯{/if}
            {#if navbarState.currentPage === "assignments"}Assignments⏰{/if}
            {#if navbarState.currentPage === "archive"}Archive📦{/if}
            <ChevronDown
              size={16}
              class={`transition-transform duration-200 ${menuOpen ? "rotate-180" : ""}`}
            />
          </span>
        </button>

        {#if menuOpen}
          <div
            class="absolute left-1/2 top-full mt-2 transform -translate-x-1/2 z-10"
          >
            <div
              class="absolute top-0 -inset-3 bg-black/40 box-shadow-lg backdrop-blur-sm p-2 rounded-b-2xl"
            ></div>
            <div
              class="relative mt-2 bg-white/60 rounded-2xl shadow p-4 space-y-2 text-center text-lg z-20 whitespace-nowrap"
            >
              <button
                on:click={() => {
                  goto("/");
                  menuOpen = false;
                }}
                class="block hover:underline w-full">Classes📖</button
              >
              <button
                on:click={() => {
                  goto("/grades");
                  menuOpen = false;
                }}
                class="block hover:underline w-full">Grades💯</button
              >
              <button
                on:click={() => {
                  goto("/assignments");
                  menuOpen = false;
                }}
                class="block hover:underline w-full">Assignments⏰</button
              >
              <button
                on:click={() => {
                  goto("/archive");
                  menuOpen = false;
                }}
                class="block hover:underline w-full">Archive📦</button
              >
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Right: buttons -->
  <div class="flex gap-4 items-center">
    {#if navbarState.currentPage === "classes"}
      <button
        class="bg-lime-500 rounded-2xl px-4 py-2 shadow cursor-pointer hover:shadow-lime-500/50"
        on:click={() => showJoinCreateModal.set(true)}
      >
        <Plus size={20} class="text-white" />
      </button>
    {/if}

    <!-- Profile dropdown -->
    <div class="relative" bind:this={profileRef}>
      <button
        class="bg-white/60 rounded-2xl px-4 py-2 shadow cursor-pointer hover:shadow-white/50 flex items-center gap-1"
        on:click={() => (profileOpen = !profileOpen)}
      >
        <User size={20} />
        <ChevronDown
          size={16}
          class={`transition-transform duration-200 ${profileOpen ? "rotate-180" : ""}`}
        />
      </button>

      {#if profileOpen}
        <div
          class="absolute left-0 top-full mt-3 transform -translate-x-1/2 z-10"
        >
          <div
            class="absolute top-0 -inset-3 bg-black/40 box-shadow-lg backdrop-blur-sm p-2 rounded-b-2xl"
          ></div>
          <div
            class="relative mt-2 bg-white/60 rounded-2xl shadow p-4 space-y-2 text-center text-lg z-20 whitespace-nowrap"
          >
            <button
              on:click={() => {
                goto("/profile");
                profileOpen = false;
              }}
              class="block hover:underline w-full"
            >
              Edit Profile ✏️
            </button>
            <button
              on:click={() => {
                alert("Settings not implemented");
                profileOpen = false;
              }}
              class="block hover:underline w-full"
            >
              Settings ⚙️
            </button>
            <button
              on:click={handleLogout}
              class="block hover:underline w-full"
            >
              Logout 🚪
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</nav>
