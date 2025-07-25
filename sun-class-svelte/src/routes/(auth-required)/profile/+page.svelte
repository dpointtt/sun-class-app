<script lang="ts">
  import { User, Mail, Settings } from "@lucide/svelte";
  import { navbarState } from "$lib/stores/state.svelte";
  import type { PageProps } from "./$types";
  import { enhance } from "$app/forms";

  navbarState.currentPage = "profile";

  let { data }: PageProps = $props();

  let originalName = $derived(data.user.name);
  // svelte-ignore state_referenced_locally
    let name = $state(originalName);
  let email = data.user.email;

  let hasChanges = $derived(name.trim() !== originalName.trim() && name.trim() !== "");
</script>

<div class="w-full h-full flex justify-center p-6">
  <div
    class="bg-black/50 box-shadow-lg backdrop-blur-sm p-3 rounded-2xl w-full max-w-screen-md"
  >
    <div class="bg-white/70 rounded-2xl p-6">
      <div class="flex justify-between items-start mb-6">
        <div class="text-gray-900">
          <h1 class="text-3xl font-bold -ms-2">👤 Your Profile</h1>
          <p class="text-md text-gray-700">Manage your personal information</p>
        </div>
      </div>

      <div class="bg-white/80 p-6 rounded-xl shadow mb-6 space-y-6">
        <div class="flex items-center gap-4">
          <div
            class="bg-gray-300 rounded-full w-20 h-20 flex items-center justify-center text-gray-700 text-4xl shadow-inner"
          >
            <User size="48" />
          </div>
          <div class="text-gray-900">
            <h2 class="text-xl font-semibold">{originalName}</h2>
            <p class="text-gray-600 flex items-center gap-2">
              <Mail size="16" />
              {email}
            </p>
          </div>
        </div>

        <form 
        method="POST" 
        action="/profile?/edit_profile" 
        use:enhance={() => {
          return async ({update}) => {
            update({reset: false});
          };
        }}
        >
          <div class="flex flex-col gap-4 mt-6">
            <div class="flex flex-col">
              <label class="text-gray-700 text-sm mb-1" for="name_input"
                >Full Name</label
              >
              <input
                id="name_input"
                type="text"
                name="name"
                bind:value={name}
                class="px-4 py-2 rounded-lg border bg-white/80 focus:outline-none focus:ring-2 focus:ring-lime-400 text-gray-900"
              />
            </div>

            <div class="flex flex-col">
              <label class="text-gray-700 text-sm mb-1" for="email_input"
                >Email Address</label
              >
              <input
                id="email_input"
                type="text"
                name="email"
                value={email}
                disabled
                class="px-4 py-2 rounded-lg border bg-gray-100 text-gray-600 cursor-not-allowed"
              />
            </div>

            <div class="flex flex-col">
              <button
                class={`px-4 py-2 rounded font-semibold shadow transition 
                  ${
                    hasChanges
                      ? "bg-lime-500 text-white hover:bg-lime-600"
                      : "bg-gray-300 text-white opacity-50 cursor-not-allowed"
                  }`}
                disabled={!hasChanges}
              >
                Save Changes
              </button>
            </div>
          </div>
        </form>
      </div>

      <div class="bg-white/80 p-6 rounded-xl shadow space-y-4">
        <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
          <Settings size="20" /> Account Actions
        </h2>
        <div class="flex flex-col gap-3">
          <button
            class="px-4 py-2 bg-red-500 text-white rounded shadow hover:bg-red-600"
          >
            Logout
          </button>
        </div>
      </div>
    </div>
  </div>
</div>
