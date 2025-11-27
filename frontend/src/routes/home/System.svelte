<script>
    import { Card, P } from "flowbite-svelte";
    import { ServerSolid } from "flowbite-svelte-icons";
    import { onMount, onDestroy } from "svelte";

    let cpuUsages = [];
    let systemData = null;
    let loading = true;
    let error = null;
    let intervalId;

    onMount(async () => {
        // await getSystemData();
        intervalId = setInterval(getSystemData, 1000);
    });

    onDestroy(() => {
        if (intervalId) {
            clearInterval(intervalId);
        }
    });

    async function getSystemData() {
        try {
            const response = await fetch('/api/home');
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            systemData = await response.json();

            // Преобразуем объект cpu_usages в массив
            if (systemData.data.cpu && systemData.data.cpu.cpu_usages) {
                cpuUsages = Object.entries(systemData.data.cpu.cpu_usages)
                    .map(([name, usage]) => ({
                        name: name,
                        usage: usage
                    }))
                    .sort((a, b) => {
                        // Сортируем по номеру CPU (cpu0, cpu1, cpu2...)
                        const numA = parseInt(a.name.replace('cpu', ''));
                        const numB = parseInt(b.name.replace('cpu', ''));
                        return numA - numB;
                    });
            }

            console.log('CPU usages:', cpuUsages);
            error = null;
        } catch (err) {
            console.error('Error fetching system data:', err);
            error = err.message;
        } finally {
            loading = false;
        }
    }
</script>

<Card class="p-4 sm:p-6 md:p-8" size="lg">
    {#if loading}
        <P class="text-gray-500">Загрузка данных...</P>
    {:else if error}
        <P class="text-red-500">Ошибка: {error}</P>
        <button
                class="mt-2 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                on:click={getSystemData}
        >
            Повторить попытку
        </button>
    {:else}
        <a href="/">
            <h5 class="mb-2 text-2xl font-semibold tracking-tight text-gray-900 dark:text-white">
                Статус сервера
                <span class="text-sm text-green-500 ml-2">●</span>
            </h5>
        </a>

        {#if cpuUsages && cpuUsages.length > 0}
            <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-3 mb-4">
                {#each cpuUsages as cpu (cpu.name)}
                    <div class="flex items-center gap-1 flex-col p-2 bg-gray-50 dark:bg-gray-800 rounded-lg">
                        <ServerSolid class="h-5 w-5 {cpu.usage > 80 ? 'text-red-700 dark:text-red-400' : cpu.usage > 50 ? 'text-orange-500' : 'text-green-600'}"/>
                        <P class="text-xs {cpu.usage > 80 ? 'text-red-700 dark:text-red-400' : cpu.usage > 50 ? 'text-orange-500' : 'text-green-600'} font-medium">
                            {cpu.name}: {cpu.usage.toFixed(1)}%
                        </P>
                    </div>
                {/each}
            </div>

            <!-- Общая статистика -->
            <div class="mt-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <P class="font-semibold text-gray-900 dark:text-white mb-2">Общая статистика CPU:</P>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                    <div>
                        <P class="text-gray-500">Ядер:</P>
                        <P class="font-medium">{cpuUsages.length}</P>
                    </div>
                    <div>
                        <P class="text-gray-500">Средняя:</P>
                        <P class="font-medium">
                            {(cpuUsages.reduce((sum, cpu) => sum + cpu.usage, 0) / cpuUsages.length).toFixed(1)}%
                        </P>
                    </div>
                    <div>
                        <P class="text-gray-500">Маx:</P>
                        <P class="font-medium text-red-600">
                            {Math.max(...cpuUsages.map(cpu => cpu.usage)).toFixed(1)}%
                        </P>
                    </div>
                    <div>
                        <P class="text-gray-500">Min:</P>
                        <P class="font-medium text-green-600">
                            {Math.min(...cpuUsages.map(cpu => cpu.usage)).toFixed(1)}%
                        </P>
                    </div>
                </div>
            </div>
        {:else}
            <P class="text-gray-500">Нет данных о загрузке CPU</P>
        {/if}
    {/if}
</Card>