package <PACKAGE>;

import com.hypixel.hytale.server.core.plugin.JavaPlugin;
import com.hypixel.hytale.server.core.plugin.JavaPluginInit;
import javax.annotation.Nonnull;
import java.util.logging.Logger;

public class Main extends JavaPlugin {
    private static final Logger LOGGER = Logger.getLogger("<MOD_NAME>");

    public Main(@Nonnull JavaPluginInit init) {
        super(init);
        LOGGER.info("Main initialized");
    }

    @Override
    protected void setup() {
        LOGGER.info("Main setup");
    }

    @Override
    protected void shutdown() {
        LOGGER.info("Main shutdown");
    }
}
