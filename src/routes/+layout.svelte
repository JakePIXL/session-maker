<script lang="ts">
    import { page } from "$app/stores";
    import { Toaster } from "svelte-french-toast";
    import "../app.css";

    interface NavItem {
        href: string;
        label: string;
        exact?: boolean;
    }

    const navigation: NavItem[] = [
        { href: "/", label: "Dashboard", exact: true },
        { href: "/history", label: "History" },
    ];

    function isActive(navItem: NavItem): boolean {
        if (navItem.exact) {
            return $page.url.pathname === navItem.href;
        }
        return $page.url.pathname.startsWith(navItem.href);
    }
</script>

<div class="min-h-screen bg-gray-100">
    <nav class="bg-indigo-600 text-white shadow-md">
        <div class="container mx-auto px-4">
            <div class="flex items-center justify-between h-16">
                <div class="flex items-center">
                    <div class="flex-shrink-0">
                        <span class="text-xl font-bold">Dibik'aandaagozi</span>
                    </div>
                    <div class="ml-10 flex items-baseline space-x-4">
                        {#each navigation as navItem}
                            <a
                                href={navItem.href}
                                class={isActive(navItem)
                                    ? "bg-indigo-800 px-3 py-2 rounded-md text-sm font-medium"
                                    : "px-3 py-2 rounded-md text-sm font-medium hover:bg-indigo-700"}
                                aria-current={isActive(navItem)
                                    ? "page"
                                    : undefined}
                            >
                                {navItem.label}
                            </a>
                        {/each}
                    </div>
                </div>
            </div>
        </div>
    </nav>

    <main>
        <slot />
    </main>

    <Toaster />
</div>
