<script>
    import ShowSourceScriptModal from "./ShowSourceScriptModal.svelte";

    async function inertData(id) {
        let response = await fetch(`/api/index-data-queries/insert-data`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json',
            },
            body: JSON.stringify({
                id: id
            })
        });
        let data = await response.json();
        console.log(data);
        console.log(Date.now());
        if (data.success) {
            console.log("success");
        }
    }

    import {Button} from 'flowbite-svelte'

    let {indexDataQueries} = $props();
    console.log(indexDataQueries);
</script>
{#each indexDataQueries as indexQuery}
    <div class="flex flex-row justify-between">
        <h5 class="dark:text-white">Data source id: {indexQuery.data_source_id}</h5>
        <ShowSourceScriptModal queryText={indexQuery.query}></ShowSourceScriptModal>
    </div>
<!--    <tr class="font-semibold text-gray-900 dark:text-white">-->
<!--        <td class="px-6 py-3 text-base">-->
<!--            Data source id: {indexQuery.data_source_id}-->
<!--        </td>-->
<!--        <td class="font-semibold text-gray-900 dark:text-white">-->
<!--            <Button color="gray" onclick={()=>inertData(indexQuery.id)}-->
<!--                    class="cursor-pointer text-sm text-blue-500 hover:text-blue-700 dark:text-blue-500 dark:hover:text-blue-700">-->
<!--                Insert data to index-->
<!--            </Button>-->
<!--        </td>-->
<!--    </tr>-->
{/each}