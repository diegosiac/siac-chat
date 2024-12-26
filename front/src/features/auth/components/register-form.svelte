<script lang="ts">
	import * as Card from '@/ui/card';
	import { Label } from '@/ui/label';
	import { Input } from '@/ui/input';
	import { Button } from '@/ui/button';
	import { ScrollArea } from '@/ui/scroll-area';
</script>

<Card.Root>
  <ScrollArea class="h-[324px]">
	<Card.Header>
		<Card.Title>Crear tu cuenta</Card.Title>
		<Card.Description>Completa el siguiente formulario para crear tu cuenta.</Card.Description>
	</Card.Header>
	<Card.Content class="space-y-2">
			<form>
				<div class="space-y-1">
					<Label for="current">Nombre Completo</Label>
					<Input id="current" type="text" />
				</div>
				<div class="space-y-1">
					<Label for="new">Nombre de usuario</Label>
					<Input id="new" type="text" />
				</div>
				<div class="space-y-1">
					<Label for="current">Correo electrónico</Label>
					<Input id="current" type="email" />
				</div>
				<div class="space-y-1">
					<Label for="new">Contraseña</Label>
					<Input id="new" type="password" />
				</div>
			</form>
    </Card.Content>
  </ScrollArea>
	<Card.Footer class="justify-end">
		<Button>Registrase</Button>
	</Card.Footer>
</Card.Root>
