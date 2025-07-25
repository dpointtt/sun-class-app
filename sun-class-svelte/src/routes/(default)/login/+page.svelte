<script lang="ts">
  import { goto } from "$app/navigation";
  // import { login, register } from "../api/auth";
  import { login, register } from "$lib/api/auth";

  let isLogin = true;
  let email = "";
  let password = "";
  let fullName = "";

  async function handleSubmit(event: Event) {
    event.preventDefault();
    try {
      if (isLogin) {
        await login(email, password);
      } else {
        await register(fullName, email, password);
      }
      goto("/");
    } catch (err) {
      alert(err);
    }
  }

  function toggleMode() {
    isLogin = !isLogin;
  }
</script>

<div class="min-h-screen flex items-center justify-center px-4">
  <div class="bg-black/50 box-shadow-lg backdrop-blur-sm p-6 rounded-2xl">
    <form
      on:submit={handleSubmit}
      class="bg-white/80 p-8 rounded-2xl shadow max-w-md w-full space-y-4"
    >
      <h2 class="text-2xl font-semibold text-center">
        {isLogin ? "Log in to🌻SunClass📖" : "Register an🌻SunClass📖 account"}
      </h2>

      {#if !isLogin}
        <input
          type="text"
          placeholder="Full Name"
          class="w-full p-2 rounded border"
          bind:value={fullName}
          required
        />
      {/if}

      <input
        type="email"
        placeholder="Email"
        class="w-full p-2 rounded border"
        bind:value={email}
        required
      />

      <input
        type="password"
        placeholder="Password"
        class="w-full p-2 rounded border"
        bind:value={password}
        required
      />

      <button
        type="submit"
        class="w-full bg-black text-white py-2 rounded-xl hover:bg-opacity-80 transition"
      >
        {isLogin ? "Log In" : "Register"}
      </button>

      <p class="text-sm text-center">
        {isLogin ? "Don't have an account?" : "Already have an account?"}
        <button class="underline cursor-pointer ml-1" on:click={toggleMode}>
          {isLogin ? "Register" : "Log In"}
        </button>
      </p>
    </form>
  </div>
</div>
