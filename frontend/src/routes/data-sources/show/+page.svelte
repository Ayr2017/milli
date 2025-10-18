<script>
    import { Section } from "flowbite-svelte-blocks";
    import { Button, Toast, Card,   P, Table, TableBody, TableBodyRow, TableBodyCell, Badge } from "flowbite-svelte";
    import { onMount } from "svelte";
    import { CheckCircleSolid } from "flowbite-svelte-icons";
    import {slide} from "svelte/transition";


    let dataSourceData = null;
    let loading = true;
    let error = null;

    onMount(async () => {

        console.log("Section mounted");
        let params = new URLSearchParams(document.location.search);
        let id = params.get("id");

        try {
            const response = await fetch(`/api/data-sources/${id}`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            console.log("Fetched data:", data);
            dataSourceData = data.data_source;
            dsTestName = dataSourceData.host;

            console.log("Fetched data:", dataSourceData);
        } catch (err) {
            console.error("Error fetching index data:", err);
            error = err.message;
        } finally {
            loading = false;
        }
    });
    function addDocuments(){
        prompt("Enter the document ID to add to the index:");
    }

    async function textConnection() {
        let testSource = await fetch(`/api/data-sources/test`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                "id": dataSourceData.id,
                "host": dataSourceData.host,
                "database": dataSourceData.database
            })
        }).then(response => {
            if(response.status === 200){
                testBtnColor = "green";
                toastStatus = true;
            } else {
                testBtnColor = "red";
            }
            return response.json()
        });
    }

    let testBtnColor = "alternative";
    let toastStatus = false;
    let dsTestName = '';
</script>

<div class="mx-auto container my-2">
    <Section>

        <Toast color="green" class="fixed top-4 right-4
 z-500 outline" dismissable={!!dsTestName} transition={slide} bind:toastStatus>
            {#snippet icon()}
                <CheckCircleSolid class="h-5 w-5" />
                <span class="sr-only">Check icon</span>
            {/snippet}
            Data source {dsTestName} checked successfully.
        </Toast>
        {#if loading}
            <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">
                <P>Loading index information...</P>
            </Card>
        {:else if error}
            <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">
                <P class="text-red-600">Error: {error}</P>
                <P>
                    <a href="/data-sources" class="text-blue-600 hover:text-blue-900">Go back to data sources</a>
                </P>
            </Card>
        {:else if dataSourceData}
            <div class="d-flex flex gap-4">
                <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">
                    <!-- Basic Information -->
                    <div class="mb-6">
                        <h3 class="text-xl dark:text-amber-100 mb-3">Basic Information</h3>
                        <Table>
                            <TableBody>
                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">ID</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.id}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Name</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.name}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Host</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.host}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Database</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.database}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Username</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.username}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Password</TableBodyCell>
                                    <TableBodyCell>••••••••</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Port</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.port}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Database Path</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.database_path}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Database Name</TableBodyCell>
                                    <TableBodyCell>{dataSourceData.database_name}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Database Type</TableBodyCell>
                                    <TableBodyCell>
                                        <Badge color="blue">{dataSourceData.database_type}</Badge>
                                    </TableBodyCell>
                                </TableBodyRow>
                            </TableBody>
                        </Table>
                    </div>
                    <div class="mt-6">
                        <Button href="/data-sources" class="mr-2">Back</Button>
                        <Button color="alternative" href="/">Go Home</Button>
                        <Button color="{testBtnColor}" onclick="{()=>textConnection()}">Test connection</Button>
                    </div>
                </Card>
            </div>
        {:else}
            <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">
                <P>No data available</P>
                <P>
                    <a href="/data-sources" class="text-blue-600 hover:text-blue-900">Data sources</a>
                </P>
            </Card>
        {/if}
    </Section>
</div>