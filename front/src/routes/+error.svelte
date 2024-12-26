<script>
	import { page } from '$app/stores';
</script>

<section class="flex size-full flex-col items-center justify-center gap-3 bg-background rounded-lg">
	<h1 class="text-3xl font-bold">{$page.status}</h1>
	<h2 class="text-3xl font-bold">Ups! Algo salió mal</h2>
	<p>{$page.error?.message}</p>
</section>
