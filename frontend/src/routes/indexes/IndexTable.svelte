<script lang="ts">
    import { Button, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Toast} from "flowbite-svelte";
    import { TrashBinOutline, FileDocSolid, CheckCircleSolid, ExclamationCircleSolid } from "flowbite-svelte-icons";
    import { Section } from "flowbite-svelte-blocks";

    import { slide } from "svelte/transition";
    import { onMount } from 'svelte';

    let data = [];
    let indexName = '';
    let toastStatus = false;


    onMount(async () => {
        const response = await fetch('/api/indexes');
        data = await response.json();
        console.log(data);
    });

    function timeout() {
        setTimeout(function () {
            toastStatus = false;
            indexName = '';
        }, 5000);
    }

    async function deleteIndex(index: string){
        fetch('/api/indexes/' + index, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                'Method': 'DELETE',
            },
            body: JSON.stringify({
                uid: index
            }),
        }).then(function(response){
            console.log(response);
            return response.json();
        }).then(function(response){
            document.querySelector('#' + index).remove();
            indexName = index;
            toastStatus = true;
            timeout();
        }).catch(function(error){
            console.log(error);
        });
    }



</script>

<Section class="py-4 sm:py-6 md:py-8">

<Toast color="green" class="fixed top-4 right-4
 z-500 outline" dismissable={!!indexName} transition={slide} bind:toastStatus>
    {#snippet icon()}
        <CheckCircleSolid class="h-5 w-5" />
        <span class="sr-only">Check icon</span>
    {/snippet}
    Index {indexName} removed successfully.
</Toast>

<Table>
    <TableHead>
        <TableHeadCell>Index name</TableHeadCell>
        <TableHeadCell>Primary key</TableHeadCell>
        <TableHeadCell>Created at</TableHeadCell>
        <TableHeadCell></TableHeadCell>
    </TableHead>
    <TableBody>
        {#each data as datum}
        <TableBodyRow id={datum.uid}>
            <TableBodyCell>{datum.uid}</TableBodyCell>
            <TableBodyCell>{datum.primary_key ?? 'default' }</TableBodyCell>
            <TableBodyCell>{datum.created_at}</TableBodyCell>
            <TableBodyCell>
                <Button pill={true} outline={true} class="p-2! border-blue-600 hover:cursor-pointer" size="xl" href="/indexes/show?uid={datum.uid}" >
                    <FileDocSolid class="text-blue-600 h-6 w-6" />
                </Button>
                <Button pill={true} outline={true} class="p-2! border-red-600 hover:cursor-pointer" size="xl" onclick={() => deleteIndex(datum.uid)} >
                    <TrashBinOutline class="text-red-600 h-6 w-6" />
                </Button>
            </TableBodyCell>
        </TableBodyRow>
            {/each}
    </TableBody>
</Table>
</Section>

