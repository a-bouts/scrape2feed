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
    <table class="table is-bordered is-fullwidth">
      <thead>
        <tr>
          <th>Id</th>
          <th>Title</th>
          <th>Link</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="feed in feeds" :key="feed.id">
          <td><a :href="feedLink(feed)">{{ feed.id }}</a></td>
          <td>{{ feed.title }}</td>
          <td><a :href="feed.link">{{ feed.link }}</a></td>
          <td><button @click="remove(feed)">Delete</button></td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
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
    feedLink(feed) {
      return "/api/v1/feeds/" + feed.id + "/content"
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
</style>
