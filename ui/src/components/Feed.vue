<template>
  <div class="container">
    <div class="columns">
      <div class="column">
        <div class="field is-horizontal">
          <div class="field-label is-normal">
            <label class="label">Url</label>
          </div>
          <div class="field-body">
            <div class="field">
              <p class="control is-expanded">
                <input class="input" type="text" placeholder="url" v-model="url">
              </p>
            </div>
          </div>
        </div>
        <div class="field is-horizontal">
          <div class="field-label is-normal">
            <label class="label">Title</label>
          </div>
          <div class="field-body">
            <div class="field">
              <p class="control is-expanded">
                <input class="input" type="text" placeholder="url" v-model="feed.title">
              </p>
            </div>
          </div>
        </div>
      </div>
      <div class="column">
        <div class="field is-horizontal">
          <div class="field-label is-normal">
            <label class="label">Node Selector</label>
          </div>
          <div class="field-body">
            <div class="field is-expanded">
              <div class="field has-addons">
                <p class="control is-expanded">
                  <input class="input" type="text" v-model="feed.nodeSelector">
                </p>
                <p class="control">
                  <a class="button" @click="selectTitle">Set</a>
                </p>
              </div>
            </div>
          </div>
        </div>
        <div class="field is-horizontal">
          <div class="field-label is-normal">
            <label class="label">Title Selector</label>
          </div>
          <div class="field-body">
            <div class="field is-expanded">
              <div class="field has-addons">
                <p class="control is-expanded">
                  <input class="input" type="text" placeholder="url" v-model="feed.titleSelector">
                </p>
                <p class="control">
                  <a class="button is-static" @click="selectTitle">Set</a>
                </p>
              </div>
            </div>
          </div>
        </div>
        <div class="field is-horizontal">
          <div class="field-label is-normal">
            <label class="label">Link Selector</label>
          </div>
          <div class="field-body">
            <div class="field">
              <p class="control is-expanded">
                <input class="input" type="text" placeholder="url" v-model="feed.linkSelector">
              </p>
            </div>
          </div>
        </div>
      </div>
      <div class="column">
        <p class="buttons">
          <button class="button is-fullwidth is-primary" @click="add">Add</button>
        </p>
      </div>
    </div>

    <div>
      <figure class="image is-16by9">
        <iframe class="has-ratio" width="100%" id='iframe' ref='iframe' :src="downloadUrl" @load="getTitle"></iframe>
      </figure>
    </div>
  </div>
</template>

<script>
import * as $ from 'jquery'

export default {
  name: 'Feed',
  props: {
    url: String
  },
  data() {
    return {
      styles: {
        hover: {
          background: "#ffeb0d",
          color: "black"
        },
        title: {
          background: "#006dcc",
          color: "white"
        }
      },
      downloadUrl: "",
      feed: {
        title: "",
        link: "",
        nodeSelector: "",
        titleSelector: "",
        linkSelector: ""
      },
      state: undefined,
      hoverElements: [],
      hoverStylesBackup: [],
      selectedTitles: [],
      selectedTitlesStylesBackup: [],
    }
  },
  created() {
    this.feed.link = this.url
    this.downloadUrl = '/api/v1/download/?url=' + encodeURI(this.url)
  },
  mounted() {
  },
  methods: {
    getTitle() {
      this.feed.title = $('iframe').contents()[0].title
    },
    selectTitle() {
      this.state = "STATE_SELECTING"
      $('iframe').contents().on('mouseenter mouseleave', '*', this.onIframeElementHover)
      $('iframe').contents().on('click', '*', this.onSelectTitle)
    },
    applyStyles(list, styles) {
      for (var i = 0 ; i < list.length ; i++) {
        this.hoverElements.push(list[i])
        var backup = {}
        for (const k of Object.keys(styles)) {
          backup[k] = list[i].style[k]
        }
        this.hoverStylesBackup.push(backup)

        for (const [k, v] of Object.entries(styles)) {
          list[i].style[k] = v
        }
      }
    },
    restoreStyles() {
      for (var i = 0 ; i < this.hoverElements.length ; i++) {
        for (const [k, v] of Object.entries(this.hoverStylesBackup[i])) {
          this.hoverElements[i].style[k] = v
        }
      }
      this.hoverElements = []
      this.hoverstylesBackup = []
    },
    onIframeElementHover(event) {
      if (this.state == "STATE_SELECTING") {
        if (event.type == 'mouseenter') {

          // On supprime les styles des elements
          this.restoreStyles()

          // On récupère le chemin de l'élement courant
          var p = event.target
          var path = p.nodeName
          while (p.parentElement) {
            p = p.parentElement
            path = p.nodeName + " > " + path
          }

          this.feed.nodeSelector = path.replaceAll(" > ", " ")

          // On selectionne tous ses compères
          var list = $('iframe').contents().find(path)

          this.applyStyles(list, this.styles.hover)

          event.stopPropagation()
        }
        else { // mouseleave
          this.restoreStyles()
        }
      }
    },
    onSelectTitle(event) {
      event.stopPropagation()
      event.preventDefault()

      this.state = ""

      this.selectedTitles = this.hoverElements
      this.restoreStyles()
      this.applyStyles(this.selectedTitles, this.styles.title)

      return false
    },
    add() {

      const options = {
          method: 'POST',
          body: JSON.stringify(this.feed),
          headers: {
              'Content-Type': 'application/json'
          }
      }

      fetch('api/v1/feeds', options).then(() => {
        this.$router.go(-1)
      })
    }
  }
}
</script>

<style scoped>

</style>
