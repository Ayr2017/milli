<script lang="ts">
    import { Button, Modal, Label, Input, Select } from "flowbite-svelte";
    let open = $state(false);
    const HTTP_STATUS_OK = 200;

    let name = $state('');
    let host = $state('');
    let database = $state('');
    let username = $state('');
    let password = $state('');
    let port = $state(5432);
    let database_path = $state('');
    let database_name = $state('');
    let database_type = $state('postgresql');
    let error = $state("");

    const databaseTypes = [
        { value: 'postgresql', name: 'PostgreSQL' },
        { value: 'mysql', name: 'MySQL' },
        { value: 'sqlite', name: 'SQLite' },
        { value: 'mongodb', name: 'MongoDB' }
    ];

    function handleSubmit() {
        if (!name.trim() || !host.trim() || !database.trim() || !username.trim() || !password.trim()) {
            error = "Пожалуйста, заполните все обязательные поля";
            return;
        }

        const formData = {
            name: name.trim(),
            host: host.trim(),
            database: database.trim(),
            username: username.trim(),
            password: password.trim(),
            port: port,
            database_path: database_path.trim(),
            database_name: database_name.trim(),
            database_type: database_type
        };

        fetch('/api/data-sources', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(formData)
        }).then(response => {
            return response.json()
        })
            .then(data => {
                if(data.code === HTTP_STATUS_OK) {
                    close();
                    clearForm();
                } else {
                    error = data.message || "Произошла ошибка при создании источника данных";
                    console.info(data);
                }
            })
            .catch(error => {
                error = "Ошибка сети или сервера";
                console.error('Error:', error);
            });
    }

    function clearForm() {
        name = '';
        host = '';
        database = '';
        username = '';
        password = '';
        port = 5432;
        database_path = '';
        database_name = '';
        database_type = 'postgresql';
        error = '';
    }

    function close() {
        open = false;
        error = '';
    }

</script>

<Button onclick={() => (open = true)}>Создать источник данных</Button>

<Modal form bind:open={open} size="lg" class="outline">
    <form onsubmit={handleSubmit}>
    <div class="flex flex-col space-y-6">
        <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Создание нового источника данных</h3>
        
        {#if error}
            <Label color="red">{error}</Label>
        {/if}
        
        <div class="grid grid-cols-2 gap-4">
            <Label class="space-y-2">
                <span>Название *</span>
                <Input
                    id="name"
                    placeholder="Введите название источника"
                    bind:value={name}
                    type="text"
                    name="name"
                    required />
            </Label>
            
            <Label class="space-y-2">
                <span>Хост *</span>
                <Input
                    id="host"
                    placeholder="localhost"
                    bind:value={host}
                    type="text"
                    name="host"
                    required />
            </Label>
        </div>
        
        <div class="grid grid-cols-2 gap-4">
            <Label class="space-y-2">
                <span>База данных *</span>
                <Input
                    id="database"
                    placeholder="Имя базы данных"
                    bind:value={database}
                    type="text"
                    name="database"
                    required />
            </Label>
            
            <Label class="space-y-2">
                <span>Порт *</span>
                <Input
                    id="port"
                    placeholder="5432"
                    bind:value={port}
                    type="number"
                    name="port"
                    min="1"
                    max="65535"
                    required />
            </Label>
        </div>
        
        <div class="grid grid-cols-2 gap-4">
            <Label class="space-y-2">
                <span>Имя пользователя *</span>
                <Input
                    id="username"
                    placeholder="Имя пользователя БД"
                    bind:value={username}
                    type="text"
                    name="username"
                    required />
            </Label>
            
            <Label class="space-y-2">
                <span>Пароль *</span>
                <Input
                    id="password"
                    placeholder="Пароль"
                    bind:value={password}
                    type="password"
                    name="password"
                    required />
            </Label>
        </div>
        
        <Label class="space-y-2">
            <span>Тип базы данных</span>
            <Select
                id="database_type"
                bind:value={database_type}
                items={databaseTypes}
                name="database_type" />
        </Label>
        
        <Label class="space-y-2">
            <span>Путь к базе данных</span>
            <Input
                id="database_path"
                placeholder="Путь к файлу БД (для SQLite)"
                bind:value={database_path}
                type="text"
                name="database_path" />
        </Label>
        
        <Label class="space-y-2">
            <span>Имя базы данных</span>
            <Input
                id="database_name"
                placeholder="Альтернативное имя базы"
                bind:value={database_name}
                type="text"
                name="database_name" />
        </Label>
        
        <div class="flex space-x-4 justify-between">
            <Button color="gray" onclick={close}>Отмена</Button>
            <Button type="submit">Создать источник</Button>
        </div>
    </div>
    </form>
</Modal>