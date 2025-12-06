<script lang="ts">
    import {Section} from "flowbite-svelte-blocks";
    import {Card, Toast} from "flowbite-svelte";
    import {Button} from "flowbite-svelte";
    import IndexTable from "./IndexTable.svelte";
    import CreateIndexModal from "./CreateIndexModal.svelte";
</script>

<div class=" mx-auto container my-2"> <!-- Контейнер -->
    <Section>
        <CreateIndexModal/>
    </Section>
    <IndexTable/>
</div>
