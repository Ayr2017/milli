<script lang="ts">
    import { Section } from "flowbite-svelte-blocks";
    import {Card, Toast} from "flowbite-svelte";
    import { Button } from "flowbite-svelte";
    import IndexTable from "./IndexTable.svelte";
    import CreateIndexModal from "./CreateIndexModal.svelte";
</script>

<div class=" mx-auto container my-2"> <!-- Контейнер -->
    <Section >
        <CreateIndexModal/>
<!--        <Button size="xs" color="alternative" class="cursor-pointer" href="/indexes/create">-->
<!--            <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">-->
<!--                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-7 7V5"/>-->
<!--            </svg>-->
<!--            Create!-->
<!--        </Button>-->
    </Section>
    <IndexTable/>
<!--    <Section >-->
<!--        <Card href="/indexes" class="p-4 sm:p-6 md:p-8">-->
<!--            <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">Statistic</h5>-->
<!--            <p class="flex justify-between leading-tight font-normal text-gray-700 dark:text-gray-400">-->
<!--                <span>Indexes:</span>-->
<!--                <span>10</span>-->
<!--            </p>-->
<!--            <p class="flex justify-between leading-tight font-normal text-gray-700 dark:text-gray-400">-->
<!--                <span>Documents:</span>-->
<!--                <span>100</span>-->
<!--            </p>-->
<!--            <p class="flex justify-between leading-tight font-normal text-gray-700 dark:text-gray-400">-->
<!--                <span>Requests(24h):</span>-->
<!--                <span>10M</span>-->
<!--            </p>-->
<!--            <p class="flex justify-between leading-tight font-normal text-gray-700 dark:text-gray-400">-->
<!--                <span>Avg time:</span>-->
<!--                <span>1m</span>-->
<!--            </p>-->
<!--        </Card>-->
<!--    </Section>-->

</div>
