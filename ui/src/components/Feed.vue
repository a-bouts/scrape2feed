<template>
  <div class="container">
    <section ref="form" class="hero is-small">
      <div class="hero-body">
        <div class="columns is-vcentered">
          <div class="column is-hidden-mobile">
            <div class="field is-horizontal">
              <div class="field-label is-small">
                <label class="label">Url</label>
              </div>
              <div class="field-body">
                <div class="field">
                  <p class="control is-expanded">
                    <input class="input is-small" readonly type="text" placeholder="url" :value="url">
                  </p>
                </div>
              </div>
            </div>
            <div class="field is-horizontal">
              <div class="field-label is-small">
                <label class="label">Title</label>
              </div>
              <div class="field-body">
                <div class="field">
                  <p class="control is-expanded">
                    <input class="input is-small" type="text" placeholder="title" v-model="feed.title">
                  </p>
                </div>
              </div>
            </div>
          </div>
          <div class="column is-hidden-mobile">
            <div class="field is-horizontal">
              <div class="field-label is-small">
                <label class="label">Node Selector</label>
              </div>
              <div class="field-body">
                <div class="field is-expanded">
                  <div class="field has-addons">
                    <p class="control is-expanded">
                      <input class="input is-small" type="text" v-model="feed.nodeSelector">
                    </p>
                  </div>
                </div>
              </div>
            </div>
            <div class="field is-horizontal">
              <div class="field-label is-small">
                <label class="label">Title Selector</label>
              </div>
              <div class="field-body">
                <div class="field is-expanded">
                  <div class="field has-addons">
                    <p class="control is-expanded">
                      <input class="input is-small" type="text" v-model="feed.titleSelector">
                    </p>
                    <p class="control">
                      <a class="button is-small" :class="{'is-info': state == 'STATE_SELECTING'}" @click="selectTitle">Set</a>
                    </p>
                  </div>
                </div>
              </div>
            </div>
            <div class="field is-horizontal">
              <div class="field-label is-small">
                <label class="label">Link Selector</label>
              </div>
              <div class="field-body">
                <div class="field">
                  <p class="control is-expanded">
                    <input class="input is-small" type="text" v-model="feed.linkSelector">
                  </p>
                </div>
              </div>
            </div>
          </div>
          <div class="column is-1">
            <div class="columns is-mobile">
              <div class="column is-hidden-desktop">
                <p class="buttons">
                  <button class="button is-fullwidth is-small" :class="{'is-info': state == 'STATE_SELECTING'}" @click="selectTitle">Set</button>
                </p>
              </div>
              <div class="column">
                <p class="buttons">
                  <button class="button is-fullwidth is-primary is-small" @click="add">Add</button>
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <div>
      <iframe width="100%" id='iframe' ref='iframe' :src="downloadUrl" @load="getTitle"></iframe>
    </div>
  </div>
</template>

<script>
//import $ from "jquery"

export default {
  name: 'Feed',
  props: {
    url: String
  },
  data() {
    return {
      styles: {
        containerHover: {
          background: "#fc9c1e",
          color: "black"
        },
        hover: {
          background: "#ffeb0d",
          color: "black"
        },
        title: {
          background: "#006dcc",
          color: "white"
        },
        selectedContainer: {
          background: "#b5dafc",
          color: "black"
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
      hover: {
        elements: [],
        stylesBackup: []
      },
      hoverContainer: {
        elements: [],
        stylesBackup: []
      },
      selected: {
        elements: [],
        stylesBackup: []
      },
      selectedContainer: {
        elements: [],
        stylesBackup: []
      }
    }
  },
  created() {
    this.feed.link = this.url
    this.downloadUrl = '/api/v1/download/?url=' + encodeURI(this.url)
  },
  mounted() {
    this.$refs.iframe.height = window.innerHeight - this.$refs.form.clientHeight - 10
  },
  methods: {
    getTitle() {
      this.feed.title = $('iframe').contents()[0].title
    },
    selectTitle() {
      this.state = "STATE_SELECTING"
      this.restoreStyles(this.selected)
      this.restoreStyles(this.selectedContainer)
      $('iframe').contents().on('mouseenter mouseleave', '*', this.onIframeElementHover)
      $('iframe').contents().on('click', '*', this.onSelectTitle)
    },
    applyStyles(list, styles, state) {
      for (var i = 0 ; i < list.length ; i++) {
        state.elements.push(list[i])
        var backup = {}
        for (const k of Object.keys(styles)) {
          backup[k] = list[i].style ? list[i].style[k] : undefined
        }
        state.stylesBackup.push(backup)

        for (const [k, v] of Object.entries(styles)) {
          list[i].style[k] = v
        }
      }
    },
    restoreStyles(state) {
      for (var i = 0 ; i < state.elements.length ; i++) {
        for (const [k, v] of Object.entries(state.stylesBackup[i])) {
          state.elements[i].style[k] = v
        }
      }
      state.elements = []
      state.stylesBackup = []
    },
    onIframeElementHover(event) {
      if (this.state == "STATE_SELECTING") {
        if (event.type == 'mouseenter') {

          // On supprime les styles des elements
          this.restoreStyles(this.hover)
          this.restoreStyles(this.hoverContainer)

          // On récupère le chemin de l'élement courant
          var path = this.getFullPath(event.target)

          // On selectionne tous ses compères
          var list = $('iframe').contents().find(path)

          // On détecte le plus vieux parents distinct
          var nodes = this.findNodes(list)

          this.applyStyles(list, this.styles.hover, this.hover)
          if (nodes.length > 1 && nodes[0] !== list[0]) {
            this.applyStyles(nodes, this.styles.containerHover, this.hoverContainer)
          }

          event.stopPropagation()
        }
        else { // mouseleave
          this.restoreStyles(this.hover)
          this.restoreStyles(this.hoverContainer)
        }
      }
    },
    findNodes(list) {
      if(list.length == 0) {
        return list
      }

      let parents = list.parent()

      if (parents.length == list.length) {
        return this.findNodes(parents)
      }

      return list
    },
    onSelectTitle(event) {
      event.stopPropagation()
      event.preventDefault()

      this.state = ""

      let elements = this.hover.elements
      let containerElements = this.hoverContainer.elements
      this.restoreStyles(this.hover)
      this.restoreStyles(this.hoverContainer)
      this.applyStyles(elements, this.styles.title, this.selected)
      this.applyStyles(containerElements, this.styles.selectedContainer, this.selectedContainer)

      // On récupère le chemin de l'élement courant
      var nodePath = this.getFullPath(event.target)
      var titlePath = ""
      var linkPath = ""

      // On selectionne tous ses compères
      var list = $('iframe').contents().find(nodePath)

      // On détecte le plus vieux parents distinct
      var nodes = this.findNodes(list)

      // il y a bien un parent
      if (nodes.length > 1 && nodes[0] !== list[0]) {
        nodePath = this.getFullPath(nodes[0])
        titlePath = this.getPath(list[0], nodes[0])
      }

      // Recherche un lien
      let link = this.findLink(list[0], nodes[0])
      if (link) {
        linkPath = this.getPath(link, nodes[0])
      }

      this.feed.nodeSelector = nodePath.replaceAll(" > ", " ")
      this.feed.titleSelector = titlePath.replaceAll(" > ", " ")
      this.feed.linkSelector = linkPath.replaceAll(" > ", " ")

      return false
    },
    findLink(element, parent) {
      var p = element
      console.log(p.nodeName)
      if (p.nodeName === "A") {
        return p
      }
      while (p.parentElement && p.parentElement != parent) {
        p = p.parentElement
        if (p.nodeName == "A") {
          return p
        }
      }
    },
    getPath(element, parent) {
      if (element === parent) {
        return
      }

      var p = element
      var path = p.nodeName
      while (p.parentElement && p.parentElement != parent) {
        p = p.parentElement
        path = p.nodeName + " > " + path
      }

      return path
    },
    getFullPath(element) {
      var p = element
      var path = p.nodeName
      while (p.parentElement) {
        p = p.parentElement
        path = p.nodeName + " > " + path
      }

      return path
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
