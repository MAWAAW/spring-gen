package {{PACKAGE_NAME}}.controller;

import {{PACKAGE_NAME}}.security.JwtUtil;
import {{PACKAGE_NAME}}.repository.UserRepository;
import jakarta.validation.Valid;
import lombok.Data;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/auth")
public class AuthController {

    private final UserRepository repo;
    private final PasswordEncoder encoder;
    private final JwtUtil jwt;

    public AuthController(UserRepository repo, PasswordEncoder encoder, JwtUtil jwt) {
        this.repo = repo;
        this.encoder = encoder;
        this.jwt = jwt;
    }

    @PostMapping("/login")
    public String login(@Valid @RequestBody LoginRequest loginRequest) {
        var user = repo.findByUsername(loginRequest.getUsername())
                .orElseThrow(() -> new RuntimeException("Bad credentials"));

        if (!encoder.matches(loginRequest.getPassword(), user.getPassword())) {
            throw new RuntimeException("Bad credentials");
        }

        return jwt.generateToken(
                org.springframework.security.core.userdetails.User
                        .withUsername(user.getUsername())
                        .password(user.getPassword())
                        .roles(user.getRole().name())
                        .build()
        );
    }

    @Data
    static class LoginRequest {
        private String username;
        private String password;
    }
}
