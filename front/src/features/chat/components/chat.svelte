<script lang="ts">
	import { Button } from '@/ui/button';
	import { Input } from '@/ui/input';
	import { ScrollArea } from '@/ui/scroll-area';
	import { Separator } from '@/ui/separator';
	import { Send } from 'lucide-svelte';
	import { CardChat } from '.';

	const messages = [
		{
			id: 1,
			user: 'User 1',
			message:
				'lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
		},
		{
			id: 2,
			user: 'User 2',
			message:
				'lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
		},
		{
			id: 3,
			user: 'User 3',
			message: 'Hola, soy User 3'
		},
		{
			id: 4,
			user: 'User 4',
			message: 'Hola, soy User 4'
		},
		{
			id: 1,
			user: 'User 1',
			message:
				'lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
		},
		{
			id: 2,
			user: 'User 2',
			message:
				'lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
		},
		{
			id: 3,
			user: 'User 3',
			message: 'Hola, soy User 3'
		},
		{
			id: 4,
			user: 'User 4',
			message: 'Hola, soy User 4'
		},
		{
			id: 1,
			user: 'User 1',
			message:
				'lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
		},
		{
			id: 2,
			user: 'User 2',
			message:
				'lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.'
		},
		{
			id: 3,
			user: 'User 3',
			message: 'Hola, soy User 3'
		},
		{
			id: 4,
			user: 'User 4',
			message: 'Hola, soy User 4'
		}
	];
</script>

<div class="relative flex size-full flex-col bg-background">
	<ScrollArea class="h-[400px] flex-grow">
		<header class="sticky left-0 top-0 rounded-xl bg-background/70 p-4 px-6 backdrop-blur-2xl">
			<div>
				<h1 class="text-3xl font-semibold">Chat General</h1>
				<p class="text-sm text-muted-foreground">
					<span>22 Mensajes</span>
					,
					<span>10 Usuarios Online</span>
				</p>
			</div>
		</header>
		<div class="flex flex-col gap-4 px-6 pt-4">
			{#each messages as message}
				<CardChat user={message.user} isMe={message.user === 'User 1'}>
					{message.message}
				</CardChat>
			{/each}
		</div>
	</ScrollArea>
	<footer class="flex items-center gap-3 p-4">
		<Input placeholder="Escribe tu mensaje" />

		<Separator class="h-7 min-h-0 rounded-full" orientation="vertical" />

		<Button size="icon">
			<Send class="relative right-px top-px size-5" />
		</Button>
	</footer>
</div>
