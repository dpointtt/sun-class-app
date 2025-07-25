<script lang="ts">
  let { children } = $props();
  import "../app.css";

  const getBackgroundImage = () => {
    const hour = new Date().getHours();
    if (hour >= 6 && hour < 12) return "/morning.jpg";
    if (hour >= 12 && hour < 18) return "/day.jpg";
    if (hour >= 18 && hour < 21) return "/evening.jpg";
    return "/night.jpg";
  };
</script>

<div
  class="fixed top-0 left-0 w-full h-full bg-cover bg-center -z-10"
  style="background-image: url({getBackgroundImage()})"
></div>

<div class="relative w-full h-full">
  {@render children()}
</div>
