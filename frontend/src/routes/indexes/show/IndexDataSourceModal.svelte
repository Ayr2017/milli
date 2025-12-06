<script lang="ts">
    import {Textarea, Button, Modal, Label, P, Select, Input, Checkbox} from "flowbite-svelte";
    import {onMount} from "svelte";

    const {indexUid} = $props();

    let textareaprops = {
        id: "query",
        name: "query",
        label: "Your query",
        rows: 4,
        placeholder: "SELECT * FROM users",
    };
    let query = $state("SELECT * FROM users");
    let formModal = $state(false);
    let error = $state("");
    let testResult = $state("");
    let dataSources = $state([]);
    let dataSourceId = $state("");

    onMount(() => {
        fetch("/api/data-sources")
            .then((res) => res.json())
            .then((res) => {
                dataSources = res.data_sources;
            });
    });

    function onaction({action, data}: { action: string; data: FormData }) {
        error = "";
        // Check the data validity, return false to prevent dialog closing; anything else to proceed
        if (action === "login" && (data.get("password") as string)?.length < 4) {
            error = "Password must have at least 4 characters";
            return false;
        }
    }

    function testQuery() {
        fetch(`/api/index-data-queries/test?uid=${indexUid}&data_source_id=${dataSourceId}&query=${query}`)
            .then((res) => {
                if (res.ok) {
                    return res.json()
                } else {
                    console.error(res);
                }
            })
            .then((res) => {
                testResult = JSON.stringify(res.result, null, 2);
                console.log(testResult);
            })
    }

    function save() {
        fetch(`/api/index-data-queries`,{
            method: "POST",
            body: JSON.stringify({
                index_uid: indexUid,
                data_source_id: dataSourceId,
                query: query,
            }),
            headers: {
                "Content-Type": "application/json",
            },
        }).then((res) => {
            if (res.ok) {
                formModal = false;
                query = "";
                dataSourceId = "";
                testResult = "";
                error = "";
            } else {
                console.error(res);
            }
        })
    }
</script>

<Button size="xs" color="green" class="cursor-pointer" outline onclick={() => (formModal = true)}>Add new query</Button>

<Modal form bind:open={formModal} size="xs" {onaction}>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Write query</h3>
        {#if error}
            <Label color="red">{error}</Label>
        {/if}

        <Label class="space-y-2">
            <span>Data Source</span>
            <Select required bind:value={dataSourceId} class="w-full">
                {#each dataSources as ds}
                    <option value={ds.id}>{ds.name}</option>
                {/each}
            </Select>
        </Label>

        <Label class="space-y-2">
            <span>Query</span>
            <Textarea {...textareaprops} class="w-full" required bind:value={query}/>
        </Label>
        <Button onclick={()=>save()} type="button" value="save" class="cursor-pointer">Save</Button>
        <Button onclick={()=>testQuery()} type="button" color="blue" class="cursor-pointer">Test query</Button>
        <pre><code class="language-json text-sm leading-relaxed">{testResult}</code>
        </pre>
    </div>
</Modal>