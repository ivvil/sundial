<div class="card card-bordered bg-base-100 shadow-xl w-96">
  <figure>
	<img src={src} alt={alt} title={title}>
  </figure>
  <div class="card-body">
	<b>{title}</b>
	<p>{author}</p>
  </div>
</div>

<script lang="ts">
  export let src: string, title: string, author: string;
  let alt: string;

  alt = "Album art for: " + title;
</script>

