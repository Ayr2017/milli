<script lang="ts">
    import { Button, Modal, Label, Input, Checkbox } from "flowbite-svelte";
    let open = $state(false);
    const HTTP_STATUS_OK = 200;

    let name = $state('');
    let pkey = $state('');

    function handleSubmit() {
        if (!name.trim()) return;
        name = name.replace(" ", "_").toLowerCase();
        pkey = pkey.replace(" ", "_").toLowerCase();

        fetch('/api/indexes', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name, pkey })
        }).then(response => {
            return response.json()
        })
            .then(data => {
                if(data.code === HTTP_STATUS_OK) {
                    open = true;
                    name = '';
                    pkey = '';
                } else {
                    console.info(data);
                }
            })
            .catch(error => {
                console.error('Error:', error);
            });
    }

    let formModal = $state(false);
    let error = $state("");

    function onaction({ action, data }: { action: string; data: FormData }) {
    }

</script>

<Button onclick={() => (open = true)}>Create index</Button>

<Modal form bind:open={open} size="xs" {onaction} class="outline">
    <form onsubmit={handleSubmit}>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Type original index name</h3>
        {#if error}
            <Label color="red">{error}</Label>
        {/if}
        <Label class="space-y-2">
            <span>Index name</span>
            <Input
                    id="name"
                    placeholder="Enter index name"
                    bind:value={name}
                    type="text"
                    name="name"
                    required />
        </Label>
        <Label class="space-y-2">
            <span>Primary key</span>
            <Input
                    id="primary_key"
                    placeholder="Enter pk name"
                    bind:value={pkey}
                    type="text"
                    name="primary_key"
                    required />
        </Label>
        <div class="space-y-4 justify-between">
            <Button color="gray" >Close</Button>
            <Button type="submit">Save index</Button>
            </div>
    </div>
    </form>
</Modal>