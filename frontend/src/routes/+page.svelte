<script lang="ts">
    import {Card, P} from "flowbite-svelte";
    import {ClipboardCheckSolid} from "flowbite-svelte-icons";
    import System from "./home/System.svelte";
</script>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8"> <!-- Контейнер -->
    <P size="3xl" class="mb-3" color="text-gray-500 dark:text-gray-400">
        Статусы системы
    </P>
</div>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 grid grid-cols-1 gap-4 xl:grid-cols-2 2xl:grid-cols-3">
    <System></System>

    <Card class="p-4 sm:p-6 md:p-8">
        <ClipboardCheckSolid class="mb-3 h-8 w-8 text-purple-700 dark:text-purple-500"/>
        <a href="/">
            <h5 class="mb-2 text-2xl font-semibold tracking-tight text-gray-900 dark:text-white">Количество
                индексов: </h5>
        </a>
        <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">Всего 99 индексов. </p>
        <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">Давай больше, давай покрупнее!</p>
    </Card>
</div>