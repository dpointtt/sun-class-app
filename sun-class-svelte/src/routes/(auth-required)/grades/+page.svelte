<script lang="ts">
    import { navbarState } from "$lib/stores/state.svelte";

    navbarState.currentPage = "grades";
</script>

<div class="p-6 bg-white/70 rounded-2xl shadow">
    📊 Your grades will show here.
</div>