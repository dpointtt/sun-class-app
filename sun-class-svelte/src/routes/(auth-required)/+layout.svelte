<script lang="ts">
  import NavBar from './Navbar.svelte';
  import NewClassModal from './NewClassModal.svelte';
  let { children } = $props();
</script>

<NavBar></NavBar>

<NewClassModal />

<div class="mt-24 px-6">
  {@render children()}
</div>