<script>
    import { Alert } from "flowbite-svelte";

    console.log('About page');
    fetch('http://localhost:3000/api/test')
        .then(response => response.json())
        .then(data => console.log(data));
</script>

<h1>About Page</h1>
<p>This is the about page of our application.</p>

<Alert color="blue">
    <span class="font-large">Default alert!</span>
    <a href="/">Go back to home</a>
</Alert>