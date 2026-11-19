<!-- An accelerator rendered as a row of brass keys. -->
<script lang="ts">
  import { shortcutKeys } from '../shortcut';

  interface Props {
    accelerator: string;
  }

  let { accelerator }: Props = $props();
</script>

<span class="keycaps">
  {#each shortcutKeys(accelerator) as key, index (index)}
    <kbd>{key}</kbd>
  {/each}
</span>

<style>
  .keycaps {
    display: inline-flex;
    gap: 5px;
    vertical-align: middle;
  }

  kbd {
    min-width: 28px;
    padding: 3px 8px 4px;
    border: 1px solid var(--brass-dark);
    border-radius: 6px;
    background: linear-gradient(var(--brass-light), var(--brass));
    box-shadow:
      inset 0 1px 0 rgb(255 245 215 / 70%),
      0 2px 0 var(--brass-dark);
    color: var(--ink);
    font-family: var(--font-body);
    font-size: 12px;
    font-weight: 800;
    line-height: 1.2;
    text-align: center;
  }
</style>
