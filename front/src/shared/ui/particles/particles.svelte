<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  // Props del componente
  export let className: string = '';
  export let quantity: number = 100;
  export let staticity: number = 50;
  export let ease: number = 50;
  export let size: number = 0.4;
  export let refresh: boolean = false;
  export let color: string = "#ffffff";
  export let vx: number = 0;
  export let vy: number = 0;

  // Referencias a elementos HTML y contexto
  let canvasRef: HTMLCanvasElement;
  let canvasContainerRef: HTMLDivElement;
  let context: CanvasRenderingContext2D | null = null;
  let circles: Circle[] = [];
  let mousePosition = { x: 0, y: 0 };
  let mouse = { x: 0, y: 0 };
  let canvasSize = { w: 0, h: 0 };
  const dpr = typeof window !== "undefined" ? window.devicePixelRatio : 1;

  // Efecto para montar y desmontar eventos
  onMount(() => {
    if (canvasRef) {
      context = canvasRef.getContext("2d");
    }
    initCanvas();
    animate();
    window.addEventListener("resize", initCanvas);

    return () => {
      window.removeEventListener("resize", initCanvas);
    };
  });

  $: {
    onMouseMove();
  }

  $: {
    if (refresh) {
      initCanvas();
    }
  }

  function hexToRgb(hex: string): number[] {
    hex = hex.replace("#", "");
    if (hex.length === 3) {
      hex = hex.split("").map((char) => char + char).join("");
    }
    const hexInt = parseInt(hex, 16);
    const red = (hexInt >> 16) & 255;
    const green = (hexInt >> 8) & 255;
    const blue = hexInt & 255;
    return [red, green, blue];
  }

  const rgb = hexToRgb(color);

  type Circle = {
    x: number;
    y: number;
    translateX: number;
    translateY: number;
    size: number;
    alpha: number;
    targetAlpha: number;
    dx: number;
    dy: number;
    magnetism: number;
  };

  function initCanvas() {
    resizeCanvas();
    drawParticles();
  }

  function resizeCanvas() {
    if (canvasContainerRef && canvasRef && context) {
      circles = [];
      canvasSize.w = canvasContainerRef.offsetWidth;
      canvasSize.h = canvasContainerRef.offsetHeight;
      canvasRef.width = canvasSize.w * dpr;
      canvasRef.height = canvasSize.h * dpr;
      canvasRef.style.width = `${canvasSize.w}px`;
      canvasRef.style.height = `${canvasSize.h}px`;
      context.scale(dpr, dpr);
    }
  }

  function circleParams(): Circle {
    const x = Math.floor(Math.random() * canvasSize.w);
    const y = Math.floor(Math.random() * canvasSize.h);
    const translateX = 0;
    const translateY = 0;
    const pSize = Math.floor(Math.random() * 2) + size;
    const alpha = 0;
    const targetAlpha = parseFloat((Math.random() * 0.6 + 0.1).toFixed(1));
    const dx = (Math.random() - 0.5) * 0.1;
    const dy = (Math.random() - 0.5) * 0.1;
    const magnetism = 0.1 + Math.random() * 4;
    return {
      x,
      y,
      translateX,
      translateY,
      size: pSize,
      alpha,
      targetAlpha,
      dx,
      dy,
      magnetism,
    };
  }

  function drawCircle(circle: Circle, update = false) {
    if (context) {
      const { x, y, translateX, translateY, size, alpha } = circle;
      context.translate(translateX, translateY);
      context.beginPath();
      context.arc(x, y, size, 0, 2 * Math.PI);
      context.fillStyle = `rgba(${rgb.join(", ")}, ${alpha})`;
      context.fill();
      context.setTransform(dpr, 0, 0, dpr, 0, 0);

      if (!update) {
        circles.push(circle);
      }
    }
  }

  function clearContext() {
    if (context) {
      context.clearRect(0, 0, canvasSize.w, canvasSize.h);
    }
  }

  function drawParticles() {
    clearContext();
    for (let i = 0; i < quantity; i++) {
      const circle = circleParams();
      drawCircle(circle);
    }
  }

  function remapValue(
    value: number,
    start1: number,
    end1: number,
    start2: number,
    end2: number,
  ): number {
    const remapped =
      ((value - start1) * (end2 - start2)) / (end1 - start1) + start2;
    return remapped > 0 ? remapped : 0;
  }

  function animate() {
    clearContext();
    circles.forEach((circle, i) => {
      const edge = [
        circle.x + circle.translateX - circle.size,
        canvasSize.w - circle.x - circle.translateX - circle.size,
        circle.y + circle.translateY - circle.size,
        canvasSize.h - circle.y - circle.translateY - circle.size,
      ];
      const closestEdge = edge.reduce((a, b) => Math.min(a, b));
      const remapClosestEdge = parseFloat(
        remapValue(closestEdge, 0, 20, 0, 1).toFixed(2),
      );
      if (remapClosestEdge > 1) {
        circle.alpha += 0.02;
        if (circle.alpha > circle.targetAlpha) {
          circle.alpha = circle.targetAlpha;
        }
      } else {
        circle.alpha = circle.targetAlpha * remapClosestEdge;
      }
      circle.x += circle.dx + vx;
      circle.y += circle.dy + vy;
      circle.translateX +=
        (mouse.x / (staticity / circle.magnetism) - circle.translateX) / ease;
      circle.translateY +=
        (mouse.y / (staticity / circle.magnetism) - circle.translateY) / ease;

      drawCircle(circle, true);

      if (
        circle.x < -circle.size ||
        circle.x > canvasSize.w + circle.size ||
        circle.y < -circle.size ||
        circle.y > canvasSize.h + circle.size
      ) {
        circles.splice(i, 1);
        const newCircle = circleParams();
        drawCircle(newCircle);
      }
    });
    window.requestAnimationFrame(animate);
  }

  function onMouseMove() {
    if (canvasRef) {
      const rect = canvasRef.getBoundingClientRect();
      const { w, h } = canvasSize;
      const x = mousePosition.x - rect.left - w / 2;
      const y = mousePosition.y - rect.top - h / 2;
      const inside = x < w / 2 && x > -w / 2 && y < h / 2 && y > -h / 2;
      if (inside) {
        mouse.x = x;
        mouse.y = y;
      }
    }
  }

  function handleMouseMove(event: MouseEvent) {
    mousePosition = { x: event.clientX, y: event.clientY };
  }

  // Listener para el movimiento del mouse
  onMount(() => {
    window.addEventListener("mousemove", handleMouseMove);
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
    };
  });
</script>

<div class={className} bind:this={canvasContainerRef} aria-hidden="true">
  <canvas bind:this={canvasRef} class="size-full"></canvas>
</div>
