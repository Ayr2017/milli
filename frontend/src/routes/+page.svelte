<script lang="ts">
    import {Navbar, NavBrand, NavLi, NavUl, NavHamburger, DarkMode} from "flowbite-svelte";
</script>



<Navbar>
    <NavBrand href="/">
        <img src="https://placehold.co/600x400" class="me-3 h-6 sm:h-9" alt="Flowbite Logo" />
        <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">Milli</span>
    </NavBrand>
    <NavHamburger />
    <NavUl>
        <NavLi href="/">Home</NavLi>
        <NavLi href="/about">About</NavLi>
        <NavLi href="/about">Navbar</NavLi>
        <NavLi href="/about">Pricing</NavLi>
        <NavLi href="/about">Contact</NavLi>
        <DarkMode />
    </NavUl>
</Navbar>
