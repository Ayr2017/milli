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
        const response = await fetch('/api/data-sources');
        data = await response.json();
        console.log(data);
    });

    function timeout() {
        setTimeout(function () {
            toastStatus = false;
            indexName = '';
            console.log('Index removed');
            console.log(toastStatus);
            console.log(indexName);
        }, 5000);
    }

    async function deleteIndex(index: string){
        fetch('/api/data-sources/' + index, {
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
            <TableHeadCell>Source id</TableHeadCell>
            <TableHeadCell>Source name</TableHeadCell>
            <TableHeadCell>Created at</TableHeadCell>
            <TableHeadCell></TableHeadCell>
        </TableHead>
        <TableBody>
            {#each data as datum}
                <TableBodyRow id={datum.id}>
                    <TableBodyCell>{datum.name}</TableBodyCell>
                    <TableBodyCell>{datum.created_at}</TableBodyCell>
                    <TableBodyCell>
                        <Button pill={true} outline={true} class="p-2! border-blue-600 hover:cursor-pointer" size="xl" href="" >
                            <FileDocSolid class="text-blue-600 h-6 w-6" />
                        </Button>
                        <Button pill={true} outline={true} class="p-2! border-red-600 hover:cursor-pointer" size="xl" >
                            <TrashBinOutline class="text-red-600 h-6 w-6" />
                        </Button>
                    </TableBodyCell>
                </TableBodyRow>
            {/each}
        </TableBody>
    </Table>
</Section>

