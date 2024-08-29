
/// <reference path='./.sst/platform/config.d.ts' />

const domain = "discitapp.com"

export default $config({
  app(input) {
    return {
      name: 'discit-backend',
      removal: 'remove',
      home: 'aws',
      providers: {
        aws: { region: 'us-east-1' }
      },
      stage: input?.stage
    };
  },
  async run() {
    const { stage } = $app;
    const environment = {
      STAGE: stage,
      LOG_LEVEL: process.env.LOG_LEVEL,
      MONGO_URI: process.env.MONGO_URI,
    }

    const cron = new sst.aws.Cron('nightly-backup', {
      schedule: 'cron(0 0 * * ? *)',
      job: {
        logging: { retention: '1 week', format: 'json' },
        environment,
        architecture: 'arm64',
        runtime: 'provided.al2023',
        handler: 'bootstrap',
        bundle: 'target/lambda/nightly-backup',
      }
    });

    const api = new sst.aws.Function('api', {
      runtime: 'provided.al2023',
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      memory: '1024 MB',
      timeout: '10 minutes',
      architecture: "arm64",
      url: { cors: true, allowCredentials: true },
      logging: { retention: '1 week', format: 'json' },
      environment,
      link: []
    });

    const router = new sst.aws.Router('MyRouter', {
      invalidation: false,
      domain: `api.${domain}`,
      routes: {
        "/*": api.url
      }
    });

    return {
      url: api.url,
      route: router.url
    }
  },
});
