<script lang="ts">
	import { Separator } from '@/ui/separator';

	import { Particles } from '@/ui/particles';
	import { Chat } from '@/features/chat/components';
</script>

<section class="w-full rounded-xl bg-background">
	<Chat />
</section>

<section
	class="relative flex w-96 flex-col justify-between overflow-hidden rounded-xl bg-background p-3"
>
	<h2 class="text-balance p-4 text-center text-3xl font-bold">Bienvenido a SiacChat</h2>
	<Particles className="absolute inset-0" quantity={100} ease={80} refresh />
	<div class="z-10 flex flex-col items-center gap-6 text-sm leading-snug">
		<article class="space-y-2 px-2">
			<p>
				<span class="rounded-sm bg-orange-500 px-1 pb-0.5 font-bold"> SiacChat </span>
				es la plataforma de chat en tiempo real diseñada para ofrecer una comunicación rápida y segura.
			</p>

			<p>
				Aquí, tus mensajes son efímeros: se borran automáticamente cada 30 minutos, garantizando una
				mayor privacidad en tus conversaciones.
			</p>
		</article>

		<Separator class="max-w-20 rounded-full" />

		<article
			class="card-bg space-y-2 rounded-lg p-3 shadow-inner shadow-[#5c5c5c33] backdrop-blur-sm"
		>
			<p>
				Cualquier uso inapropiado o comportamiento abusivo podría resultar en la suspensión de tu
				cuenta.
			</p>
			<p>
				Disfruta de una experiencia de chat dinámica, pero siempre con respeto y responsabilidad :).
			</p>
		</article>
	</div>
</section>

<style>
	.card-bg {
		background: linear-gradient(140deg, #5c5c5c33, #fff0 60%),
			linear-gradient(348deg, #5c5c5c33, #fff0 39%);
	}
</style>
