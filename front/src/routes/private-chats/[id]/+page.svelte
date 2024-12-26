<script lang="ts">
	import { CardUserChat, Chat } from '@/features/chat/components';
	import { ScrollArea } from '@/ui/scroll-area';

	const array = Array.from({ length: 50 }, (_, i) => i + 1);
</script>

<section class="w-full max-w-80 overflow-hidden rounded-xl bg-background">
	<ScrollArea class="h-[580px] flex-grow">
		<header
			class="sticky left-0 top-0 z-[1] w-full rounded-xl bg-background/70 px-6 py-4 backdrop-blur-2xl"
		>
			<h1 class="text-2xl font-semibold">Chats Privados</h1>
		</header>
		<div class="flex flex-col pl-2 pr-3">
			{#each array as item}
				<CardUserChat />
			{/each}
		</div>
	</ScrollArea>
</section>

<section class="w-full rounded-xl bg-background">
	<Chat />
</section>
