<script lang="ts">
	import { cn } from '@/utils';

	export let user: string;
	export let isMe: boolean;
</script>

<article
	class={cn('space-y-1', {
		'ml-auto': isMe
	})}
>
	<p class="pl-2 text-xs font-semibold text-muted-foreground">{user}</p>
	<p
		class={cn('w-max max-w-prose rounded-lg bg-primary/10 p-2 px-3 text-sm', {
			'bg-foreground text-background': isMe
		})}
	>
		<slot />
	</p>
</article>
