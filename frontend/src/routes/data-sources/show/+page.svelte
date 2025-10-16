<script>
    import { Section } from "flowbite-svelte-blocks";
    import { Button, Card,   P, Table, TableBody, TableBodyRow, TableBodyCell, Badge } from "flowbite-svelte";
    import { onMount } from "svelte";

    let indexData = null;
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
            indexData = data;
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
</script>

<div class="mx-auto container my-2">
    <Section>
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
        {:else if indexData}
            <div class="d-flex flex gap-4">
                <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">
                    <!-- Basic Information -->
                    <div class="mb-6">
                        <h3 class="text-xl dark:text-amber-100 mb-3">Basic Information</h3>
                        <Table>
                            <TableBody>
                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">ID</TableBodyCell>
                                    <TableBodyCell>{indexData.uid}</TableBodyCell>
                                </TableBodyRow>

                                <TableBodyRow>
                                    <TableBodyCell class="font-medium">Indexing Status</TableBodyCell>
                                    <TableBodyCell>
                                        {#if indexData.stats.is_indexing}
                                            <Badge color="yellow">Indexing...</Badge>
                                        {:else}
                                            <Badge color="green">Ready</Badge>
                                        {/if}
                                    </TableBodyCell>
                                </TableBodyRow>
                            </TableBody>
                        </Table>
                    </div>
                </Card>
                <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">

                    <div class="mt-6">
                        <Button href="/data-sources" class="mr-2">Back</Button>
                        <Button color="alternative" href="/">Go Home</Button>
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