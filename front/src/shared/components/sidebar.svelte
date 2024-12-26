<script lang="ts">
	import { MessageSquare, Users } from 'lucide-svelte';
	import { Separator } from '@/ui/separator';
	import { NavLink, ProfileButton } from '.';
	import { LogoIcon } from '@/icons';
</script>

<aside class="h-[calc(100vh-24px)] rounded-xl bg-background p-4 w-min">
	<div class="flex h-full flex-col">
		<div class="flex flex-col items-center">
			<LogoIcon className="h-8 w-8" />
		</div>

    <Separator class="mt-6 mb-5 rounded-full" />

		<nav class="flex flex-grow flex-col">
			<NavLink title="Chat General" href="/">
				<MessageSquare class="h-5 w-5"></MessageSquare>
			</NavLink>

			<NavLink title="Chat Privados" href="/private-chats">
				<Users class="h-5 w-5"></Users>
			</NavLink>
		</nav>

		<ProfileButton />
	</div>
</aside>
