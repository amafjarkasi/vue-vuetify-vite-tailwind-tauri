<!-- eslint-disable no-unused-vars -->
<template>
  <v-container>
    <v-row class="text-center">
      <v-col cols="12">
        <!-- <v-img :src="require('../assets/logo.svg')" class="my-3" contain height="200" /> -->
      </v-col>
    
      <v-col class="mb-4">
        <v-alert v-model="alert" dismissible color="cyan" border="left" elevation="2" colored-border icon="mdi-twitter">
          You've got <strong>5</strong> new updates on your timeline!.
        </v-alert>
        <h1 class="display-2 font-weight-bold mb-3">
          Welcome to Vuetify
        </h1>

        <v-chip class="ma-2" color="primary" @click="invokeBackend(), alert=true">
          Primary
        </v-chip>

        <p class="subheading font-weight-regular">
          <v-rating background-color="purple lighten-3" color="purple" large></v-rating>
        </p>
      </v-col>

      <v-col class="mb-5" cols="12">
        <h2 class="justify-center flex bg-yellow-300 items-center h-screen">
          What's next?
        </h2>

        <v-row justify="center">
          <a v-for="(next, i) in whatsNext" :key="i" :href="next.href" class="subheading mx-3" target="_blank">
            {{ next.text }}
          </a>
        </v-row>
      </v-col>

      <v-col class="mb-5" cols="12">
        <h2 class="headline font-weight-bold mb-3">
          Important Links
        </h2>

        <v-row justify="center">
          <a v-for="(link, i) in importantLinks" :key="i" :href="link.href" class="subheading mx-3" target="_blank">
            {{ link.text }}
          </a>
        </v-row>
      </v-col>

      <v-col class="mb-5" cols="12">
        <h2 class="headline font-weight-bold mb-3">
          Ecosystem
        </h2>

        <v-row justify="center">
          <a v-for="(eco, i) in ecosystem" :key="i" :href="eco.href" class="subheading mx-3" target="_blank">
            {{ eco.text }}
          </a>
        </v-row>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
export default {
  name: 'HelloWorld',

  data: () => ({
    alert: false,
    ecosystem: [
      {
        text: 'vuetify-loader',
        href: 'https://github.com/vuetifyjs/vuetify-loader',
      },
      {
        text: 'github',
        href: 'https://github.com/vuetifyjs/vuetify',
      },
      {
        text: 'awesome-vuetify',
        href: 'https://github.com/vuetifyjs/awesome-vuetify',
      },
    ],
    importantLinks: [
      {
        text: 'Documentation',
        href: 'https://vuetifyjs.com',
      },
      {
        text: 'Chat',
        href: 'https://community.vuetifyjs.com',
      },
      {
        text: 'Made with Vuetify',
        href: 'https://madewithvuejs.com/vuetify',
      },
      {
        text: 'Twitter',
        href: 'https://twitter.com/vuetifyjs',
      },
      {
        text: 'Articles',
        href: 'https://medium.com/vuetify',
      },
    ],
    whatsNext: [
      {
        text: 'Explore components',
        href: 'https://vuetifyjs.com/components/api-explorer',
      },
      {
        text: 'Select a layout',
        href: 'https://vuetifyjs.com/getting-started/pre-made-layouts',
      },
      {
        text: 'Frequently Asked Questions',
        href: 'https://vuetifyjs.com/getting-started/frequently-asked-questions',
      },
    ],
  }),
  methods: {
    async invokeBackend() {
      const invoke = window.__TAURI__.invoke
      invoke('my_custom_command')
      const response = await invoke('greeting', { name: 'World' })
      console.log(response)
    },
  },
}
</script>
