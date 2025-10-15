<script lang="ts">
    import { Button, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Toast} from "flowbite-svelte";
    import { TrashBinOutline, FileDocSolid, CheckCircleSolid, ExclamationCircleSolid } from "flowbite-svelte-icons";
    import { Section } from "flowbite-svelte-blocks";

    import { slide } from "svelte/transition";
    import { onMount } from 'svelte';
    import CreateDataSourceModal from "./CreateDataSourceModal.svelte";

    let data = [];
    let dsName = '';
    let toastStatus = false;


    onMount(async () => {
        let response = await fetch('/api/data-sources');
        let json = await response.json();
        console.log(json);
        data = json.data_sources;
    });

    function timeout() {
        setTimeout(function () {
            toastStatus = false;
            dsName = '';
            console.log('Index removed');
            console.log(toastStatus);
            console.log(dsName);
        }, 5000);
    }

    async function deleteDs(index: string){
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
            dsName = index;
            toastStatus = true;
            timeout();
        }).catch(function(error){
            console.log(error);
        });
    }



</script>
<Section class="py-4 sm:py-6 md:py-8">
    <CreateDataSourceModal />

    <Toast color="green" class="fixed top-4 right-4
 z-500 outline" dismissable={!!dsName} transition={slide} bind:toastStatus>
        {#snippet icon()}
            <CheckCircleSolid class="h-5 w-5" />
            <span class="sr-only">Check icon</span>
        {/snippet}
        Data Source {dsName} removed successfully.
    </Toast>

    <Table hoverable={true}>
        <TableHead>
            <TableHeadCell>Name</TableHeadCell>
            <TableHeadCell>Host</TableHeadCell>
            <TableHeadCell>Path</TableHeadCell>
            <TableHeadCell>Type</TableHeadCell>
            <TableHeadCell>Created at</TableHeadCell>
            <TableHeadCell></TableHeadCell>
        </TableHead>
        <TableBody>
            {#each data as datum}
                <TableBodyRow id={datum.id}>
                    <TableBodyCell>{datum.name}</TableBodyCell>
                    <TableBodyCell>{datum.host}</TableBodyCell>
                    <TableBodyCell>{datum.database_path}</TableBodyCell>
                    <TableBodyCell>{datum.database_type}</TableBodyCell>
                    <TableBodyCell>{datum.created_at}</TableBodyCell>
                    <TableBodyCell>
                        <Button pill={true} outline={true} class="p-2! dark:border-blue-600 border-blue-600 hover:cursor-pointer" size="xl" href="" >
                            <FileDocSolid class="text-blue-600 h-6 w-6" />
                        </Button>
                        <Button pill={true} outline={true} onclick="deleteDs()" class="p-2! dark:border-red-600 border-red-600 hover:cursor-pointer" size="xl" >
                            <TrashBinOutline class="text-red-600 h-6 w-6" />
                        </Button>
                    </TableBodyCell>
                </TableBodyRow>
            {/each}
        </TableBody>
    </Table>
</Section>

