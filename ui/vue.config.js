module.exports = {
  devServer: {
     proxy: {
       '^/api': {
         target: 'http://localhost:8000'
         // target: 'https://scrape2feed.apps-k3s.no-cloud.fr'
       }
     }
  }
}
