<template>
  <div class="container">
    <section class="hero">
      <div class="hero-body">
        <div class="container has-text-centered">
          <p class="title">
            Add new Feed
          </p>
          <div class="subtitle">
            <div class="columns is-centered">
              <div class="column is-half">
                <div class="field has-addons has-addons-centered">
                  <div class="control is-expanded">
                    <input class="input" type='text' v-model="url" />
                  </div>
                  <div class="control" @click="add">
                    <a class="button is-info">
                      Go
                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
    <table class="table is-bordered is-stripped is-fullwidth">
      <thead>
        <tr>
          <th></th>
          <th>Title</th>
          <th>Link</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="feed in feeds" :key="feed.id">
          <td><span class="icon is-clickable has-tooltip-active" @click="copy(feedLink(feed))"><i class="far fa-clone"></i></span></td>
          <td class="is-clipped" width="50%">{{ feed.title }}</td>
          <td class="is-clipped" width="50%"><a :href="feed.link">{{ feed.link }}</a></td>
          <td><span class="icon is-clickable" @click="remove(feed)"><i class="fa fa-trash"></i></span></td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import * as bulmaToast from 'bulma-toast'

export default {
  name: 'Feeds',
  data() {
    return {
      feeds: [],
      url: ""
    }
  },
  created() {
    fetch('api/v1/feeds')
      .then(response => response.json())
      .then(feeds => this.feeds = feeds)
  },
  methods: {
    copy(text) {
      bulmaToast.toast({
        message: text + " copied!",
        type: 'is-info is-light',
        duration: 2000,
        position: "top-center",
        animate: { in: 'fadeInDown', out: 'fadeOutUp' },
      })
      navigator.clipboard.writeText(text)
    },
    feedLink(feed) {
      return window.location.origin + "/api/v1/feeds/" + feed.id
    },
    remove(feed) {
      console.log("Delete feed " + feed.id)
      fetch('api/v1/feeds/' + feed.id, {method:'DELETE'})
        .then(
          this.feeds.splice(this.feeds.indexOf(feed), 1)
        )
    },
    add() {
      console.log("add url " + this.url)
      this.$router.push({ path: `/new`, query: {url: this.url} })
    }
  }
}
</script>

<style scoped>
td {
  vertical-align: middle !important;
}
</style>
