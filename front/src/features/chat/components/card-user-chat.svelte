<script lang="ts">
	import * as Avatar from '@/ui/avatar';
</script>

<a href="/private-chats/1" class="rounded-lg px-4 py-3 transition-colors hover:bg-primary/5">
	<article class="flex items-center gap-2">
		<Avatar.Root class="z-0">
			<Avatar.Fallback>CN</Avatar.Fallback>
		</Avatar.Root>
		<div class="flex flex-col">
			<span>User 1</span>
			<span class="max-w-[190px] truncate text-sm"
				>Lorem ipsum dolor sit amet consectetur adipisicing elit. Accusantium officiis temporibus
				totam veniam</span
			>
		</div>
	</article>
</a>
