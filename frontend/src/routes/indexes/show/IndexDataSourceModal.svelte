<script lang="ts">
    import {Textarea, Button, Modal, Label,P,Select, Input, Checkbox } from "flowbite-svelte";
    import {onMount} from "svelte";

    const { indexUid } = $props();

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

    onMount(() => {
        fetch("/api/data-sources")
            .then((res) => res.json())
            .then((res) => {
                dataSources = res.data_sources;
            });
    });

    function onaction({ action, data }: { action: string; data: FormData }) {
        error = "";
        // Check the data validity, return false to prevent dialog closing; anything else to proceed
        if (action === "login" && (data.get("password") as string)?.length < 4) {
            error = "Password must have at least 4 characters";
            return false;
        }
    }

    function testQuery() {
        fetch(`/api/index-data-queries/test?uid=${indexUid}&query=${query}`)
    }
</script>

<Button onclick={() => (formModal = true)}>Form modal</Button>

<Modal form bind:open={formModal} size="xs" {onaction}>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Write query</h3>
        {#if error}
            <Label color="red">{error}</Label>
        {/if}

        <Label class="space-y-2">
            <span>Data Source</span>
            <Select>
                {#each dataSources as ds}
                    <option value={ds.uid}>{ds.name}</option>
                {/each}
            </Select>
        </Label>

        <Label class="space-y-2">
            <span>Query</span>
            <Textarea {...textareaprops} class="w-full" required bind:value={query} />
        </Label>
        <Button type="submit" value="login" class="cursor-pointer">Save</Button>
        <Button onclick={()=>testQuery()} type="button" color="blue" class="cursor-pointer">Test query</Button>
        <P>{testResult}</P>
    </div>
</Modal>