<script lang="ts">
    import { page } from "$app/stores";
    import { Toaster } from "svelte-french-toast";
    import { Tabs } from "bits-ui";
    import "../app.css";

    interface NavItem {
        href: string;
        label: string;
        value: string;
        exact?: boolean;
    }

    const navigation: NavItem[] = [
        { href: "/", label: "Dashboard", value: "dashboard", exact: true },
        { href: "/history", label: "History", value: "history" },
    ];

    function isActive(navItem: NavItem): boolean {
        if (navItem.exact) {
            return $page.url.pathname === navItem.href;
        }
        return $page.url.pathname.startsWith(navItem.href);
    }

    // Get current tab value based on the URL
    const getCurrentTab = (): string => {
        const currentPath = $page.url.pathname;
        const item = navigation.find(item => 
            (item.exact && currentPath === item.href) || 
            (!item.exact && currentPath.startsWith(item.href))
        );
        return item?.value || "dashboard";
    };

    let activeTab = getCurrentTab();
</script>

<div class="min-h-screen">
    <header class="border-b-[var(--border-width)] border-[var(--color-border)] px-6 py-6">
        <div class="container mx-auto">
            <div class="flex flex-col md:flex-row items-center justify-center">
                
                <Tabs.Root value={activeTab} class="flex items-center">
                    <Tabs.List>
                        {#each navigation as navItem}
                            <Tabs.Trigger 
                                value={navItem.value}
                            >
                                <a
                                    href={navItem.href}
                                    class="relative px-4 py-2 font-bold"
                                    data-active={isActive(navItem)}
                                    style="
                                        {isActive(navItem) 
                                            ? 'background-color: var(--color-foreground); color: var(--color-surface); box-shadow: none; transform: translate(var(--shadow-offset), var(--shadow-offset));' 
                                            : 'background-color: var(--color-surface); border: var(--border-width) solid var(--color-border); box-shadow: var(--shadow-offset) var(--shadow-offset) 0px 0px rgba(0,0,0,1);'}
                                    "
                                >
                                    {navItem.label.toUpperCase()}
                                </a>
                            </Tabs.Trigger>
                        {/each}
                    </Tabs.List>
                </Tabs.Root>
            </div>
        </div>
    </header>

    <main class="container mx-auto">
        <slot />
    </main>

    <Toaster 
        toastOptions={{
            style: `
                font-family: var(--font-mono);
                background-color: var(--color-surface);
                color: var(--color-foreground);
                border: var(--border-width) solid var(--color-border);
                border-radius: 0;
                box-shadow: var(--shadow-offset) var(--shadow-offset) 0px 0px rgba(0,0,0,1);
                padding: 12px;
            `,
        }}
    />
</div>