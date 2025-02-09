require_relative 'lib/bytebuffer/version'

Gem::Specification.new do |spec|
  spec.name = 'bytebuffer'
  spec.version = ByteBuffer::VERSION
  spec.summary = 'A gem wrapping the bytebuffer crate using FFI.'
  spec.description = 'A gem that provides a ByteBuffer class based on the bytebuffer rust crate.'
  spec.homepage = 'https://github.com/sickday/bytebuffer-rb'
  spec.license = 'BSD-3-Clause'
  spec.author = 'Patrick W.'
  spec.email = 'Sickday@pm.me'

  spec.metadata = {
    "homepage_uri" => 'https://github.com/sickday/bytebuffer-rb',
    "documentation_uri" => 'https://sickday.github.io/bytebuffer-rb/'
  }

  spec.required_ruby_version = '>= 3.1.0'

  spec.add_runtime_dependency 'ffi', '~> 1.17'

  spec.add_development_dependency 'rake', '~> 13.0'
  spec.add_development_dependency 'rake-compiler', '~> 1.2'
  spec.add_development_dependency 'rb_sys', '~> 0.9'
  spec.add_development_dependency 'rspec', '~> 3.12'

  spec.extensions = %w(ext/bytebuffer/extconf.rb)
  spec.require_paths = %w(lib)

  spec.files = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|git)/}) }
end
