<script lang="ts">
    import {Card, P} from "flowbite-svelte";
    import {ClipboardCheckSolid} from "flowbite-svelte-icons";
    import Sidebar from "./Sidebar.svelte";

    import Cpu from "./Cpu.svelte";
    import Indexes from "./Indexes.svelte";

    let content = $state(0);

    function setContent(value: number) {
        content = value;
    }
</script>

<div class="flex mx-auto h-full"> <!-- Контейнер -->
    <Sidebar bind:content {setContent}></Sidebar>
    <div class="flex flex-col w-full h-full p-2"> <!-- Контейнер -->
        {#if ((content) === 0) }
            <Cpu></Cpu>
        {:else if ((content) === 1) }
            <Indexes></Indexes>
        {/if}
    </div>
</div>