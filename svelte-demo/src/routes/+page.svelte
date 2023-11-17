<script lang="ts">
	// Wasm module
	import init from "$lib/pkg/sandbox_bevy";

	// Svelte
	import { onMount } from "svelte";

	// Variables
	let loading = false;
	let window: HTMLDivElement;
	let isDragging = false;
	let x = 0;
	let y = 0;
	let lastX = 0;
	let lastY = 0;
	let linkCopied = false;

	onMount(async () => {
		try {
			loading = true;
			await init();
			loading = false;
		} catch (error) {
			console.log("[ERROR]:");
			console.error(error);
		} finally {
			loading = false;
		}
	});

	function handleMouseDown(e: MouseEvent) {
		isDragging = true;
		lastX = e.clientX;
		lastY = e.clientY;
	}

	function handleMouseMove(e: MouseEvent) {
		if (isDragging) {
			const deltaX = e.clientX - lastX;
			const deltaY = e.clientY - lastY;
			x += deltaX;
			y += deltaY;
			lastX = e.clientX;
			lastY = e.clientY;
			if (window) window.style.transform = `translate(${x}px, ${y}px)`;
		}
	}

	function handleMouseUp() {
		isDragging = false;
	}

	function copyLink() {
		navigator.clipboard.writeText("https://huggingface.co/spaces/HugoDzz/rust-sandbox");
		linkCopied = true;
	}

</script>

<!-- Main container -->
<div on:mousemove={handleMouseMove} role="contentinfo" class="sm:flex flex-col justify-center items-center w-full">

	<!-- Window -->
	<div bind:this={window} class="hidden sm:flex flex-col justify-center items-center mt-40">
		<!-- Title container -->
		<button
			on:mousedown={handleMouseDown}
			on:mouseup={handleMouseUp}
			class="cursor-default relative flex flex-row justify-between items-center w-full bg-[#393B3D] border-b border-b-[#000002] border border-[#626264] rounded-xl rounded-b-none"
		>
			<!-- macOS dots container -->
			<div class="absolute left-2 space-x-2 flex flex-row justify-center items-center">
				<div class="rounded-full h-3 w-3 bg-[#EC695F]" />
				<div class="rounded-full h-3 w-3 bg-[#F5BE4F]" />
				<div class="rounded-full h-3 w-3 bg-[#5E5E60]" />
			</div>

			<h1 class="font-bold py-[6px] text-center w-full text-sm text-[#B6B8B9]">Hell</h1>
		</button>

		<!-- Canvas container -->
		<div class="relative bg-[#1C1E21]">
			<canvas
				id="bevy-canvas"
				on:contextmenu|preventDefault={() => {}}
				class="shadow-2xl z-10 shadow-black/50 overflow-hidden rounded-xl border border-[#626264] border-t-0 rounded-t-none"
				height="480"
				width="640"
				style="image-rendering: pixelated;"
			/>
			{#if loading}
				<div
					class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 space-x-3 flex flex-row justify-center items-center"
				>
					<div class="h-2 w-2 bg-[#B6B8B9] animate-pulse" />
					<div class="h-2 w-2 bg-[#B6B8B9] animate-pulse" />
					<div class="h-2 w-2 bg-[#B6B8B9] animate-pulse" />
				</div>
			{/if}
		</div>
	</div>

	<!-- Instructions -->
	<div class="hidden sm:flex mt-12">
		<img src="images/instructs.svg" alt="Instructions" width="250" />
	</div>

	<!-- Mobile UI-->
	<div class="flex sm:hidden flex-col w-full px-4 justify-center items-center text-center text-[#636669] mt-12">
		<p class="text-xl text-[#F0DAA1]">Looks like you're on mobile</p>
		<p class="mt-2">This demo is only available on a larger screen</p>
		<button on:click={copyLink} class="w-full py-4 bg-[#393B3D] text-[#B7B7B9] font-bold mt-8 border border-[#626264] rounded-md">
			<p>{linkCopied ? "Copied!" : "Copy the link for later"}</p>
		</button>
		<img src="images/thumbnail.png" alt="Thumbnail" class="w-full mt-4">
	</div>

	<!-- Credits -->
	<div class="mt-12 text-sm text-[#636669] hidden sm:flex flex-col justify-center items-center space-y-1">
		<p>
			Made by <a href="https://www.hugoduprez.com/" target="_blank" class="underline">Hugo</a> with
			<a href="https://www.rust-lang.org/" target="_blank" class="underline">Rust</a>
			and <a href="https://kit.svelte.dev/" target="_blank" class="underline">Svelte</a>
		</p>
		<a href="https://github.com/Hugo-Dz/rust-sandbox" target="_blank" class="underline">How to create games with Rust</a
		>
	</div>
</div>
