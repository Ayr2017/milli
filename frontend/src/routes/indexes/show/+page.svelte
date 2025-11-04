<script>
    import { Section } from "flowbite-svelte-blocks";
    import { Textarea, Button, Card, Label,  P, Table, TableHead, TableHeadCell, TableBody, TableBodyRow, TableBodyCell, Badge } from "flowbite-svelte";
    import { onMount } from "svelte";
    import IndexDataSourceModal from "./IndexDataSourceModal.svelte";

    let indexData = null;
    let loading = true;
    let error = null;
    let indexDataSources = null;
    let indexUid = '';

    let textareaprops = {
        id: "message",
        name: "message",
        label: "Your message",
        rows: 10,
        placeholder: "Leave a comment..."
    };

    onMount(async () => {
        console.log("Section mounted");
        let params = new URLSearchParams(document.location.search);
        let uid = params.get("uid");
        indexUid = uid;

        try {
            const response = await fetch(`/api/indexes/${uid}`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            console.log("Fetched data:", data);
            indexData = data;
            getIndexDataQueries(uid);
        } catch (err) {
            console.error("Error fetching index data:", err);
            error = err.message;
        } finally {
            loading = false;
        }
    });

    function getIndexDataQueries(uid){
        indexDataSources = fetch(`/api/index-data-queries?filter[index_uid]=${uid}&limit=1000`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                "Accept": "application/json",
            }
        }).then(response => {
            if (!response.ok) {
                console.log("Error fetching index data:", response);
                return null;
            } else {
                return response.json();
            }
        });
    }

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
                    <a href="/index" class="text-blue-600 hover:text-blue-900">Go back to indexes</a>
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
                                <TableBodyCell class="font-medium">UID</TableBodyCell>
                                <TableBodyCell>{indexData.uid}</TableBodyCell>
                            </TableBodyRow>
                            <TableBodyRow>
                                <TableBodyCell class="font-medium">Primary Key</TableBodyCell>
                                <TableBodyCell>{indexData.primary_key || 'Not set'}</TableBodyCell>
                            </TableBodyRow>
                            <TableBodyRow>
                                <TableBodyCell class="font-medium">Created At</TableBodyCell>
                                <TableBodyCell>{indexData.created_at}</TableBodyCell>
                            </TableBodyRow>
                            <TableBodyRow>
                                <TableBodyCell class="font-medium">Updated At</TableBodyCell>
                                <TableBodyCell>{indexData.updated_at}</TableBodyCell>
                            </TableBodyRow>
                            <TableBodyRow>
                                <TableBodyCell class="font-medium">Documents</TableBodyCell>
                                <TableBodyCell>
                                    <Badge color="blue">{indexData.stats.number_of_documents}</Badge>
                                </TableBodyCell>
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
                        <TableBody>
                            <TableBodyRow>
                                <TableBodyCell class="font-medium">Index Name</TableBodyCell>
                            </TableBodyRow>
                            <TableBodyRow>
                                <TableBodyCell class="font-medium">Index Type</TableBodyCell>
                            </TableBodyRow>
                            <TableBodyRow>
                                <TableBodyCell></TableBodyCell>
                            </TableBodyRow>
                        </TableBody>
                        <tfoot>
                        <tr class="font-semibold text-gray-900 dark:text-white">
                            <th scope="row" class="px-6 py-3 text-base">
                                <IndexDataSourceModal indexUid={indexUid}/>
                            </th>
                            <th class="font-semibold text-gray-900 dark:text-white">
                                <Button color="blue" class="mr-2" onclick="{addDocuments}">Check</Button>
                            </th>
                        </tr>
                        </tfoot>
                    </Table>
                </div>

            </Card>

            <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">
                <h2 class="text-2xl mb-4 dark:text-primary-900">Index: {indexData.uid}</h2>



                <!-- Settings -->
                <div class="mb-6">
                    <h3 class="text-xl  mb-3">Settings</h3>

                    <!-- Ranking Rules -->
                    <div class="mb-4">
                        <Label class="font-medium">Ranking Rules</Label>
                        {#if indexData.ranking_rules && indexData.ranking_rules.length > 0}
                            <div class="mt-1">
                                {#each indexData.ranking_rules as rule}
                                    <Badge color="gray" class="mr-1 mb-1">{rule}</Badge>
                                {/each}
                            </div>
                        {:else}
                            <P class="text-gray-500">No ranking rules set</P>
                        {/if}
                    </div>

                    <!-- Searchable Attributes -->
                    <div class="mb-4">
                        <Label class="font-medium">Searchable Attributes</Label>
                        {#if indexData.searchable_attributes && indexData.searchable_attributes.length > 0}
                            <div class="mt-1">
                                {#each indexData.searchable_attributes as attr}
                                    <Badge color="green" class="mr-1 mb-1">{attr}</Badge>
                                {/each}
                            </div>
                        {:else}
                            <P class="text-gray-500">No searchable attributes set</P>
                        {/if}
                    </div>

                    <!-- Filterable Attributes -->
                    <div class="mb-4">
                        <Label class="font-medium">Filterable Attributes</Label>
                        {#if indexData.filterable_attributes && indexData.filterable_attributes.length > 0}
                            <div class="mt-1">
                                {#each indexData.filterable_attributes as attr}
                                    <Badge color="purple" class="mr-1 mb-1">{attr}</Badge>
                                {/each}
                            </div>
                        {:else}
                            <P class="text-gray-500">No filterable attributes set</P>
                        {/if}
                    </div>

                    <!-- Sortable Attributes -->
                    <div class="mb-4">
                        <Label class="font-medium">Sortable Attributes</Label>
                        {#if indexData.sortable_attributes && indexData.sortable_attributes.length > 0}
                            <div class="mt-1">
                                {#each indexData.sortable_attributes as attr}
                                    <Badge color="indigo" class="mr-1 mb-1">{attr}</Badge>
                                {/each}
                            </div>
                        {:else}
                            <P class="text-gray-500">No sortable attributes set</P>
                        {/if}
                    </div>

                    <!-- Displayable Attributes -->
                    <div class="mb-4">
                        <Label class="font-medium">Displayable Attributes</Label>
                        {#if indexData.displayable_attributes && indexData.displayable_attributes.length > 0}
                            <div class="mt-1">
                                {#each indexData.displayable_attributes as attr}
                                    <Badge color="teal" class="mr-1 mb-1">{attr}</Badge>
                                {/each}
                            </div>
                        {:else}
                            <P class="text-gray-500">No displayable attributes set</P>
                        {/if}
                    </div>

                    <!-- Stop Words -->
                    <div class="mb-4">
                        <Label class="font-medium">Stop Words</Label>
                        {#if indexData.stop_words && indexData.stop_words.length > 0}
                            <div class="mt-1">
                                {#each indexData.stop_words as word}
                                    <Badge color="red" class="mr-1 mb-1">{word}</Badge>
                                {/each}
                            </div>
                        {:else}
                            <P class="text-gray-500">No stop words set</P>
                        {/if}
                    </div>

                    <!-- Synonyms -->
                    <div class="mb-4">
                        <Label class="font-medium">Synonyms</Label>
                        {#if indexData.synonyms && Object.keys(indexData.synonyms).length > 0}
                            <Table class="mt-2">
                                <TableHead>
                                    <TableHeadCell>Term</TableHeadCell>
                                    <TableHeadCell>Synonyms</TableHeadCell>
                                </TableHead>
                                <TableBody>
                                    {#each Object.entries(indexData.synonyms) as [term, synonyms]}
                                        <TableBodyRow>
                                            <TableBodyCell class="font-medium">{term}</TableBodyCell>
                                            <TableBodyCell>
                                                {#each synonyms as synonym}
                                                    <Badge color="yellow" class="mr-1 mb-1">{synonym}</Badge>
                                                {/each}
                                            </TableBodyCell>
                                        </TableBodyRow>
                                    {/each}
                                </TableBody>
                            </Table>
                        {:else}
                            <P class="text-gray-500">No synonyms set</P>
                        {/if}
                    </div>

                    <!-- Distinct Attribute -->
                    <div class="mb-4">
                        <Label class="font-medium">Distinct Attribute</Label>
                        <P>{indexData.distinct_attribute || 'Not set'}</P>
                    </div>
                </div>

                <div class="mt-6">
                    <Button href="/indexes" class="mr-2">Back to Indexes</Button>
                    <Button color="alternative" href="/">Go Home</Button>
                    <Button color="blue" onclick="{addDocuments}" class="mr-2">Add documents</Button>
                </div>
            </Card>
            </div>
        {:else}
            <Card size="lg" class="p-4 text-left sm:p-8 md:p-10 w-full">
                <P>No data available</P>
                <P>
                    <a href="/indexes" class="text-blue-600 hover:text-blue-900">Go to index</a>
                </P>
            </Card>
        {/if}
    </Section>
</div>