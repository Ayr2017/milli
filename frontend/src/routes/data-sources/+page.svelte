<script lang="ts">
    import { Section } from "flowbite-svelte-blocks";
    import {Card, Toast} from "flowbite-svelte";
    import { Button } from "flowbite-svelte";
    import DataSourcesTable from "./DataSourcesTable.svelte";
</script>

<div class=" mx-auto container my-2"> <!-- Контейнер -->
    <Section >
    <DataSourcesTable/>
    </Section>
</div>
