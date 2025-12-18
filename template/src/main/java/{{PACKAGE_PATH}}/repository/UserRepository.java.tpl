package {{PACKAGE_NAME}}.repository;

import java.util.Optional;
import org.springframework.data.jpa.repository.JpaRepository;
import {{PACKAGE_NAME}}.model.User;

public interface UserRepository extends JpaRepository<User, Long> {
    Optional<User> findByUsername(String username);
}
