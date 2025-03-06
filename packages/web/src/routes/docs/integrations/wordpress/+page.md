<script>
    import {Button} from "flowbite-svelte"
    import blueprint from "../../../../../demo_wp_blueprint.json?raw"

    let base64Blueprint = btoa(blueprint)
    let playgroundUrl = `https://playground.wordpress.net/?mode=seamless#${base64Blueprint}`
</script>

# Harper for WordPress

Unlike other grammar checkers for WordPress, Harper is designed to be fast and __get out of the way__.

<Button href={playgroundUrl} target="_blank">Demo Inside Gutenberg Playground</Button>
