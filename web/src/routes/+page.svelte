<script lang="ts">
    import DarkLightMode from "@/components/other/DarkLightMode.svelte";
</script>

<div class="flex h-screen w-full">
    <div class="text-center w-full">
        <h1 class="text-8xl mt-8 font-bold">Kontrl</h1>
        <p class="text-xl mx-auto">Making user managment great again</p>
    </div>

    <div class="absolute bottom-3 left-3">
        <DarkLightMode />
    </div>
</div>